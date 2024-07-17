# `Math Quiz Blitz`

This project presents a WEB 3.0 website built using the Vue and Tailwind frameworks for the frontend and Rust for the backend.

This is a math quiz that randomizes equations to be solved from addition, subtraction, multiplication and division. The player has one minute to solve as many equations as possible. Each good answer will add another two seconds to the time, while a negative answer will take away two seconds. Once the time is up, the player has the option to submit his or her results to the leaderboard.

Application when wrong answer is given:

![image](https://github.com/user-attachments/assets/fa0fb741-ca6e-4336-b46f-fa9800488d80)

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.

If you have made changes to your backend canister, you can generate a new candid interface with

```bash
npm run generate
```

at any time. This is recommended before starting the frontend development server, and will be run automatically any time you run `dfx deploy`.

If you are making frontend changes, you can start a development server with

```bash
npm start
```

Which will start a server at `http://localhost:8080`, proxying API requests to the replica at port 4943.

### Note on frontend environment variables

If you are hosting frontend code somewhere without using DFX, you may need to make one of the following adjustments to ensure your project does not fetch the root key in production:

- set`DFX_NETWORK` to `ic` if you are using Webpack
- use your own preferred method to replace `process.env.DFX_NETWORK` in the autogenerated declarations
  - Setting `canisters -> {asset_canister_id} -> declarations -> env_override to a string` in `dfx.json` will replace `process.env.DFX_NETWORK` with the string in the autogenerated declarations
- Write your own `createActor` constructor
