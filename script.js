const fetch = require('node-fetch');

const API_TOKEN = process.argv[2];
const SERVICE_ID = "ee4bdd61-21d2-4bde-bba4-4be716d1ed9d"; // from URL
const PROJECT_ID = "6794a5bd-f6b0-46b2-a199-506694340d97"; // from URL

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
      serviceId: SERVICE_ID,
    }
  };

  const response = await fetch("https://backboard.railway.app/graphql/v2", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${API_TOKEN}`,
    },
    body: JSON.stringify({
      query,
      variables
    }),
  });

  const data = await response.json();

  if (data.errors) {
    console.error("❌ Redeploy failed:", data.errors);
    process.exit(1);
  } else {
    console.log("✅ Redeploy successful:", data.data.serviceInstanceRedeploy);
  }
}

redeploy();
