const util = require('util');
const exec = util.promisify(require('child_process').exec);

const pathToWASM = "artifacts/cw_pepperwatch.wasm";
const instantiateFromAccount = "test1";
const chainId = "localterra";

const instantiateMSG = {
	minter: 'terra1x46rqay4d3cssq8gxxvqz8xt6nwlz4td20k38v',
	name: 'TEST',
	symbol: 'TST',
};

/// Trying to mimic our data fields
const mintMSG = { mint: {
	token_id: 'Qme5WuoHFFfvWcbs7SVDuwEvCX5QVdhqZKbbXgEAwZUgqj',
	token_uri: 'ipfs://Qme5WuoHFFfvWcbs7SVDuwEvCX5QVdhqZKbbXgEAwZUgqj',
	owner: 'terra1x46rqay4d3cssq8gxxvqz8xt6nwlz4td20k38v',
	extension: {
		name: 'test',
		description: 'test',
		image: 'ipfs://Qme5WuoHFFfvWcbs7SVDuwEvCX5QVdhqZKbbXgEAwZUgqj',
		animation_url: 'https://pepperwatch.com/v/QmSmJ2B8YA4k3n2EnyT3mWCEX39hD77duqFGhLESc3YHPJ',
		external_url: 'https://pepperwatch.com/v/QmSmJ2B8YA4k3n2EnyT3mWCEX39hD77duqFGhLESc3YHPJ',
	},
	token_key: 'somekeyencoded',
	token_key_version: 2,
}};


const askForKeyMSG = { "ask_for_key": {"media": "Qme5WuoHFFfvWcbs7SVDuwEvCX5QVdhqZKbbXgEAwZUgqj", "key": "GNEfmvn23KXvYJclh5KhaezRq4+2AYr4/unj7tMzVk0="}};
const fillTheKeyMSG = { "fill_key": {"media": "Qme5WuoHFFfvWcbs7SVDuwEvCX5QVdhqZKbbXgEAwZUgqj", "addr": "terra1x46rqay4d3cssq8gxxvqz8xt6nwlz4td20k38v", "key": "encodedGNEfmvn23KXvYJclh5KhaezRq4+2AYr4/unj7tMzVk0="}};
const withdrawMSG = { "withdraw": {} };

const execute = async (command)=>{
	let gasEstimate = null;

	try {
		const { stdout, stderr } = await exec(command);

		// console.log(stdout);
		// console.log(stderr);

		try {
			if (stderr.indexOf('gas estimate:') != -1) {
				gasEstimate = parseInt(stderr.split('estimate: ')[1], 10);
			}
		} catch(e) {

		}

		const resp = JSON.parse(stdout);

		// console.log(resp);

		resp.gasEstimate = gasEstimate;

		return resp;
	} catch(e) {
		console.error(e);

		return {gasEstimate: gasEstimate};
	}
};

const searchAttr = (resp, eventType, attrKey) => {
	let data = null;
	for (let event of resp.logs[0].events) {
		if (event.type == eventType) {
			for (let attr of event.attributes) {
				if (attr.key == attrKey) {
					data = attr.value;
				}
			}
		}
	}

	return data;
}

const run = async ()=>{
	let resp = null;
	let totalGas = 0;

	resp = await execute(`terrad tx wasm store ${pathToWASM} -y --from ${instantiateFromAccount} --output json --chain-id=${chainId} --gas=auto --fees=100000uluna --broadcast-mode=block`);
	let codeId = searchAttr(resp, 'store_code', 'code_id');

	if (resp.gasEstimate) {
		totalGas += resp.gasEstimate;
	}

	console.log('codeId', codeId);

	let cmd = `terrad tx wasm instantiate ${codeId} '${JSON.stringify(instantiateMSG)}' --from ${instantiateFromAccount} -y --output json  --chain-id=${chainId} --fees=10000uluna --gas=auto --broadcast-mode=block`;
	resp = await execute(cmd);

	let contractAddress = searchAttr(resp, 'instantiate_contract', 'contract_address');

	if (resp.gasEstimate) {
		totalGas += resp.gasEstimate;
	}

	console.log('contractAddress', contractAddress);

	cmd = `terrad tx wasm execute ${contractAddress} '${JSON.stringify(mintMSG)}' 1000000uluna --from ${instantiateFromAccount} -y --output json --chain-id=${chainId} --fees=1000000uluna --gas=auto --broadcast-mode=block`;
	resp = await execute(cmd);
	const mintGas = resp.gasEstimate;

	if (resp.gasEstimate) {
		totalGas += resp.gasEstimate;
	}
	console.log('mint gas', mintGas);



	cmd = `terrad tx wasm execute ${contractAddress} '${JSON.stringify(askForKeyMSG)}' 1000000uluna --from ${instantiateFromAccount} -y --output json --chain-id=${chainId} --fees=1000000uluna --gas=auto --broadcast-mode=block`;
	resp = await execute(cmd);
	const askGas = resp.gasEstimate;

	if (resp.gasEstimate) {
		totalGas += resp.gasEstimate;
	}
	console.log('ask the key gas', askGas);


	cmd = `terrad tx wasm execute ${contractAddress} '${JSON.stringify(fillTheKeyMSG)}' --from ${instantiateFromAccount} -y --output json --chain-id=${chainId} --fees=1322uluna --gas=auto --broadcast-mode=block`;
	resp = await execute(cmd);
	const fillGas = resp.gasEstimate;

	if (resp.gasEstimate) {
		totalGas += resp.gasEstimate;
	}
	console.log('fill the key gas', fillGas);


	// cmd = `terrad tx wasm execute ${contractAddress} '${JSON.stringify(withdrawMSG)}' 1000000uluna --from ${instantiateFromAccount} -y --output json --chain-id=${chainId} --fees=1000000uluna --gas=auto --broadcast-mode=block`;
	// resp = await execute(cmd);
	// const withdrawGas = resp.gasEstimate;

	// if (resp.gasEstimate) {
	// 	totalGas += resp.gasEstimate;
	// }
	// console.log('withdraw gas', withdrawGas);

	console.log('total gas', totalGas);

};

run();