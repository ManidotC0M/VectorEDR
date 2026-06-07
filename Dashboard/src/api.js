import axios from "axios";

const API_BASE = "http://127.0.0.1:8080";

export const getAgents = async () => {
  const res = await axios.get(`${API_BASE}/agents`);
  return res.data;
};

export const getReports = async () => {
  const res = await axios.get(`${API_BASE}/reports`);
  return res.data;
};
