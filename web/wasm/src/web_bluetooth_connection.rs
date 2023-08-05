use std::cell::RefCell;

use async_trait::async_trait;
use js_sys::{Array, Uint8Array};
use macaddr::MacAddr6;
use openscq30_lib::{
    api::connection::{Connection, ConnectionStatus},
    device_utils::{READ_CHARACTERISTIC_UUID, WRITE_CHARACTERISTIC_UUID},
    Result as LibResult,
};
use tokio::sync::{mpsc, watch};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    BluetoothDevice, BluetoothRemoteGattCharacteristic, BluetoothRemoteGattServer,
    BluetoothRemoteGattService,
};

pub struct WebBluetoothConnection {
    gatt: BluetoothRemoteGattServer,
    write_characteristic: BluetoothRemoteGattCharacteristic,
    read_characteristic: BluetoothRemoteGattCharacteristic,
    packet_receive_handler: RefCell<Option<Closure<dyn Fn()>>>,
}

impl WebBluetoothConnection {
    pub async fn new(device: BluetoothDevice) -> Result<Self, JsValue> {
        let gatt = device
            .gatt()
            .ok_or("Bluetooth device does not support GATT")?;
        let gatt: BluetoothRemoteGattServer = JsFuture::from(gatt.connect()).await?.into();
        let services: Array = JsFuture::from(gatt.get_primary_services()).await?.into();
        if services.length() == 0 {
            return Err("Gatt service not found".into());
        }
        let service: BluetoothRemoteGattService = services.get(0).into();
        let write_characteristic: BluetoothRemoteGattCharacteristic = JsFuture::from(
            service.get_characteristic_with_str(&WRITE_CHARACTERISTIC_UUID.to_string()),
        )
        .await?
        .into();
        let read_characteristic: BluetoothRemoteGattCharacteristic = JsFuture::from(
            service.get_characteristic_with_str(&READ_CHARACTERISTIC_UUID.to_string()),
        )
        .await?
        .into();

        JsFuture::from(read_characteristic.start_notifications()).await?;

        Ok(Self {
            gatt,
            write_characteristic,
            read_characteristic,
            packet_receive_handler: Default::default(),
        })
    }
}

#[async_trait(?Send)]
impl Connection for WebBluetoothConnection {
    async fn name(&self) -> LibResult<String> {
        Ok(self.gatt.device().name().unwrap_or_default())
    }

    async fn mac_address(&self) -> LibResult<MacAddr6> {
        // WebBluetooth does not provide the mac address of the device
        Ok(MacAddr6::default())
    }

    fn connection_status(&self) -> watch::Receiver<ConnectionStatus> {
        // TODO implement
        let (sender, receiver) = watch::channel(ConnectionStatus::Connected);
        receiver
    }

    async fn write_with_response(&self, data: &[u8]) -> LibResult<()> {
        // not sure why this needs to be mutable
        let mut data = data.to_owned();
        JsFuture::from(
            self.write_characteristic
                .write_value_with_response_with_u8_array(&mut data),
        )
        .await
        .unwrap(); // TODO handle error
        Ok(())
    }

    async fn write_without_response(&self, data: &[u8]) -> LibResult<()> {
        // not sure why this needs to be mutable
        let mut data = data.to_owned();
        JsFuture::from(
            self.write_characteristic
                .write_value_without_response_with_u8_array(&mut data),
        )
        .await
        .unwrap(); // TODO handle error
        Ok(())
    }

    async fn inbound_packets_channel(&self) -> LibResult<mpsc::Receiver<Vec<u8>>> {
        let (sender, receiver) = mpsc::channel(50);
        let read_characteristic = self.read_characteristic.to_owned();

        // TODO remove event listener on disconnect?
        let handler: Closure<dyn Fn()> = Closure::new(move || {
            if let Some(value) = read_characteristic.value() {
                let data = Uint8Array::new(&value.buffer().into()).to_vec();
                let sender = sender.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    sender.send(data).await.unwrap();
                });
            }
        });
        *self.packet_receive_handler.borrow_mut() = Some(handler);

        self.read_characteristic
            .set_oncharacteristicvaluechanged(Some(
                &self
                    .packet_receive_handler
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .as_ref()
                    .unchecked_ref(),
            ));
        Ok(receiver)
    }
}

impl Drop for WebBluetoothConnection {
    fn drop(&mut self) {
        self.gatt.disconnect();
    }
}