import React from "react";
import Agents from "./components/Agents";
import Reports from "./components/Reports";
import ChartView from "./components/Chart";


export default function App() {
  return (
    <div style={styles.container}>
      <h1 style={styles.title}>🛡 Vector EDR Dashboard</h1>

      <div style={styles.grid}>
        <Agents />
        <Reports />
      </div>

      <ChartView />
    </div>
  );
}

const styles = {
  container: {
    background: "#0f172a",
    minHeight: "100vh",
    color: "#e5e7eb",
    padding: 20,
    fontFamily: "Segoe UI"
  },
  title: {
    textAlign: "center",
    marginBottom: 30
  },
  grid: {
    display: "grid",
    gridTemplateColumns: "1fr 1fr",
    gap: 20
  }
};
