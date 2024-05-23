# tunl
a serverless v2ray tunnel

## Quick Start
1. [Create an API token](https://developers.cloudflare.com/fundamentals/api/get-started/create-token/) from the cloudflare dashboard.
2. Create a `.env` file based on `.env.example` and fill the values based on your tokens

| Variable            | Description                                      |
|---------------------|--------------------------------------------------|
| CLOUDFLARE_API_TOKEN | The API key retrieved from Cloudflare dashboard |

3. Deploy
```sh
$ make deploy
```

4. Modify the [xray config](./config/xray.json) and run:
```sh
$ xray -c ./config/xray.json
```
