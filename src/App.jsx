//import 'bootstrap/dist/css/bootstrap.min.css';
import { Button, ButtonGroup, Container, Table, Badge, Modal, Col, Row } from "react-bootstrap";
import { invoke,  } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";
import { debug } from "tauri-plugin-log-api";
import "./App.css";
import 'bootstrap/dist/css/bootstrap.min.css';
import { useEffect, useState } from "react";

function App() {
  const [packet_list, set_packet_list] = useState([]);

  return (
    <div className="container">
      <header>
        <h1 style={{fontSize: "350%", fontWeight: "1"}}>RealmStat</h1>
        <p>RotMG Packet Capture and Analysis</p>
      </header>
      <SnifferController set_packet_list={set_packet_list}/>
      <Container>
        <Table style={{"textAlign": "left"}} striped>
          <thead>
            <tr>
              <th style={{"width": "20%"}}>Packet #</th>
              <th>Tick ID</th>
            </tr>
          </thead>
          <tbody>
            {packet_list.map((p, i) => 
              <tr>
                <td>{i}</td>
                <td>{JSON.stringify(p)}</td>
              </tr>
            )}
          </tbody>
        </Table>
      </Container>
      <SelectDeviceModal />
    </div>
  );
}

export default App;


function SnifferController({packet_list, set_packet_list}) {
  const [collecting, set_collecting] = useState(false);
  const [aligned, set_aligned] = useState(false);
  const [read_counter, set_read_counter] = useState(0);

  //These two useEffect calls control when new packets are fetched from the backend to be displayed in the table
  useEffect(() => {
    const timer = setTimeout(() => collecting && set_read_counter(read_counter+1), 1e3);
    return () => clearTimeout(timer);
  }, [collecting, read_counter])
  useEffect(() => {
    if(collecting == true) get_packets();
  }, [read_counter]);

  //event listeners to display the aligned status of the cipher
  appWindow.listen("cipher-aligned", (e) => {
    //debug("Cipher aligned");
    //e.preventDefault();
    set_aligned(true);
  });
  appWindow.listen("cipher-misaligned", (e) => {
    //debug("Cipher misaligned");
    //e.preventDefault();
    set_aligned(false);
  });

  //Functions to start & stop packet collection
  async function start() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    await invoke("start_collection");
    set_collecting(true);
  }
  async function stop() {
    await invoke("stop_collection");
    set_collecting(false);
  }

  //Function to fetch all packets fetched and stored by the current sniffer session
  async function get_packets() {
    var packets = await invoke("get_packets");
    //packets = packets.filter(p => "NewTick" in p);
    //debug(""+JSON.stringify(packets[0]));
    //packets.map(p => debug(""+JSON.stringify(p["NewTick"])))
    set_packet_list(packets);
  }

  return (
    <Container>
      <ButtonGroup>
        <Button onClick={start} disabled={collecting} variant="success">Start</Button>
        <Button onClick={stop} disabled={!collecting} variant="danger">Stop</Button>
      </ButtonGroup>
      <br/>
      {collecting ? (
        aligned ? (
          <Badge bg="success" style={{fontSize: "120%"}}>Cipher Aligned</Badge>
        ) : (
          <Badge bg="danger" style={{fontSize: "120%"}}>Cipher Misaligned</Badge>
        )
      ) : (
        <Badge bg="secondary" style={{fontSize: "120%"}}>Cipher Paused</Badge>
      )}
    </Container>
  )
}

function SelectDeviceModal() {
  const [devices, set_devices] = useState([]);
  const [show, set_show] = useState(true);

  async function get_devices() {
    let ds = await invoke("get_devices");
    set_devices(ds);
  }
  useEffect(() => {
    get_devices();
  }, []);

  async function select_device(device_string) {
    try {
      await invoke("use_device", {deviceString: device_string});
      set_show(false);
    } catch (e) {
      debug("Error " + e);
    }
  }

  return (
    <div>
      <Modal show={show}>
        <Modal.Header><h1>Select network adapter</h1></Modal.Header>
        <Modal.Body>
          <Table>
            <thead>
              <tr>
                <th>Device Description</th>
              </tr>
            </thead>
            <tbody>
              {devices.map((d) => {
                return (
                  <tr>
                    <td>
                      <Row>
                        <Col>{d}</Col>
                        <Col style={{textAlign: "right"}} onClick={() => select_device(d)}><Button>Select</Button></Col>
                      </Row>
                    </td>
                  </tr>
                )
              })}  
            </tbody>
          </Table>
        </Modal.Body>
      </Modal>
    </div>
  )
}

