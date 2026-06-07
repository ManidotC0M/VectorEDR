import React, { useEffect, useState } from "react";
import { getReports } from "../api";

export default function Reports() {
  const [reports, setReports] = useState([]);

  useEffect(() => {
    getReports()
      .then(setReports)
      .catch(() => setReports([]));
  }, []);

  return (
    <div style={box}>
      <h2>📄 Latest Reports</h2>
      <ul>
        {reports.map((r, i) => (
          <li key={i}>
            {r.hostname} | Processes: {r.process_count} | Uptime: {r.uptime}s
          </li>
        ))}
      </ul>
    </div>
  );
}

const box = {
  background: "#020617",
  padding: 20,
  borderRadius: 10
};
