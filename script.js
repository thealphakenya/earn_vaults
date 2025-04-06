const fetch = require("node-fetch");

const API_TOKEN = process.argv[2];
const PROJECT_ID = "6794a5bd-f6b0-46b2-a199-506694340d97";
const SERVICE_ID = "ee4bdd61-21d2-4bde-bba4-4be716d1ed9d";

async function redeploy() {
  const query = `
    mutation RedeployService($input: ServiceInstanceRedeployInput!) {
      serviceInstanceRedeploy(input: $input) {
        id
        status
      }
    }
  `;

  const variables = {
    input: {
      projectId: PROJECT_ID,
      serviceId: SERVICE_ID
    }
  };

  const res = await fetch("https://backboard.railway.app/graphql/v2", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "Authorization": `Bearer ${API_TOKEN}`,
    },
    body: JSON.stringify({ query, variables }),
  });

  const json = await res.json();
  if (json.errors) {
    console.error("❌ Redeploy failed:", json.errors);
    process.exit(1);
  } else {
    console.log("✅ Redeploy successful:", json.data.serviceInstanceRedeploy);
  }
}

redeploy();
