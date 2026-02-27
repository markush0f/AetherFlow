import { Client } from 'pg';
const client = new Client({ connectionString: "postgres://postgres:postgres@localhost:5432/aetherflow" });
client.connect();
client.query("SELECT * FROM agent", (err, res) => {
  if (err) throw err;
  console.log(res.rows);
  client.end();
});
