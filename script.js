const https = require('https');

const TOKEN = process.argv[2];
const ENVIRONMENT_ID = "e9bef5e3-26ae-4868-b99a-22f6d3ae47f4";
const SERVICE_ID = "ee4bdd61-21d2-4bde-bba4-4be716d1ed9d";

// GraphQL mutation to trigger redeploy
const data = JSON.stringify({
  query: `
    mutation ServiceInstanceRedeploy {
      serviceInstanceRedeploy(
        environmentId: "${ENVIRONMENT_ID}"
        serviceId: "${SERVICE_ID}"
      )
    }
  `
});

const options = {
  hostname: 'backboard.railway.app',
  path: '/graphql/v2',
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
    'Content-Length': data.length,
    'Authorization': `Bearer ${TOKEN}`
  }
};

const req = https.request(options, res => {
  let body = '';
  res.on('data', chunk => {
    body += chunk;
  });
  res.on('end', () => {
    try {
      const response = JSON.parse(body);
      if (response.errors) {
        console.error('❌ Deployment failed:', response.errors);
        process.exit(1);
      } else {
        console.log('✅ Railway deployment triggered successfully:', response.data);
      }
    } catch (err) {
      console.error('❌ Failed to parse the response:', err);
      process.exit(1);
    }
  });
});

req.on('error', error => {
  console.error('❌ Request error:', error);
  process.exit(1);
});

req.write(data);
req.end();
