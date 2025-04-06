const fetch = require('node-fetch');

const token = process.argv[2];
const serviceId = "ee4bdd61-21d2-4bde-bba4-4be716d1ed9d"; // Your Railway service ID

const query = `
mutation {
  serviceInstanceRedeploy(id: "${serviceId}") {
    id
    status
  }
}`;

fetch('https://backboard.railway.app/graphql/v2', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
    Authorization: `Bearer ${token}`,
  },
  body: JSON.stringify({ query }),
})
  .then(res => res.json())
  .then(data => {
    if (data.errors) {
      console.error("❌ Redeploy failed:", data.errors);
      process.exit(1);
    } else {
      console.log("✅ Redeploy successful:", data.data.serviceInstanceRedeploy);
    }
  })
  .catch(err => {
    console.error("❌ Network error:", err);
    process.exit(1);
  });
