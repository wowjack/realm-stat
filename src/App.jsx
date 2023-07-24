//import 'bootstrap/dist/css/bootstrap.min.css';
import { Button, ButtonGroup, Container, Table, Badge } from "react-bootstrap";
import { invoke,  } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";
import { debug } from "tauri-plugin-log-api";
import "./App.css";
import 'bootstrap/dist/css/bootstrap.min.css';
import { useState } from "react";

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
    </div>
  );
}

export default App;


function SnifferController({packet_list, set_packet_list}) {
  const [collecting, set_collecting] = useState(false);
  const [aligned, set_aligned] = useState(false);

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

  async function start() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    await invoke("start_collection");
    set_collecting(true);
  }

  async function stop() {
    await invoke("stop_collection");
    set_collecting(false);
  }

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
        <Button onClick={get_packets}>Print Packets</Button>
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

