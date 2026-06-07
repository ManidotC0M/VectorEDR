import React, { useEffect, useState } from "react";
import { getAgents } from "../api";

export default function Agents() {
  const [agents, setAgents] = useState([]);

  useEffect(() => {
    getAgents()
      .then(setAgents)
      .catch(() => setAgents([]));
  }, []);

  return (
    <div style={box}>
      <h2>🖥 Active Agents</h2>
      <table style={table}>
        <thead>
          <tr>
            <th>Hostname</th>
            <th>OS</th>
            <th>Last Seen</th>
          </tr>
        </thead>
        <tbody>
          {agents.map((a, i) => (
            <tr key={i}>
              <td>{a.hostname}</td>
              <td>{a.os}</td>
              <td>{a.last_seen}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

const box = {
  background: "#020617",
  padding: 20,
  borderRadius: 10
};

const table = {
  width: "100%",
  borderCollapse: "collapse"
};
