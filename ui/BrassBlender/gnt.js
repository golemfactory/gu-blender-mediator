
$(function() {

    const RINKEBY = '4';

    const MAINNET = '1';

    const ABI = {
        GNT: '[{"constant":true,"inputs":[],"name":"name","outputs":[{"name":"","type":"string"}],"payable":false,"type":"function"},{"constant":true,"inputs":[],"name":"golemFactory","outputs":[{"name":"","type":"address"}],"payable":false,"type":"function"},{"constant":true,"inputs":[],"name":"totalSupply","outputs":[{"name":"","type":"uint256"}],"payable":false,"type":"function"},{"constant":false,"inputs":[{"name":"_master","type":"address"}],"name":"setMigrationMaster","outputs":[],"payable":false,"type":"function"},{"constant":true,"inputs":[],"name":"decimals","outputs":[{"name":"","type":"uint8"}],"payable":false,"type":"function"},{"constant":false,"inputs":[{"name":"_value","type":"uint256"}],"name":"migrate","outputs":[],"payable":false,"type":"function"},{"constant":false,"inputs":[],"name":"finalize","outputs":[],"payable":false,"type":"function"},{"constant":false,"inputs":[],"name":"refund","outputs":[],"payable":false,"type":"function"},{"constant":true,"inputs":[],"name":"migrationMaster","outputs":[{"name":"","type":"address"}],"payable":false,"type":"function"},{"constant":true,"inputs":[],"name":"tokenCreationCap","outputs":[{"name":"","type":"uint256"}],"payable":false,"type":"function"},{"constant":true,"inputs":[{"name":"_owner","type":"address"}],"name":"balanceOf","outputs":[{"name":"","type":"uint256"}],"payable":false,"type":"function"},{"constant":false,"inputs":[{"name":"_agent","type":"address"}],"name":"setMigrationAgent","outputs":[],"payable":false,"type":"function"},{"constant":true,"inputs":[],"name":"migrationAgent","outputs":[{"name":"","type":"address"}],"payable":false,"type":"function"},{"constant":true,"inputs":[],"name":"fundingEndBlock","outputs":[{"name":"","type":"uint256"}],"payable":false,"type":"function"},{"constant":true,"inputs":[],"name":"totalMigrated","outputs":[{"name":"","type":"uint256"}],"payable":false,"type":"function"},{"constant":true,"inputs":[],"name":"symbol","outputs":[{"name":"","type":"string"}],"payable":false,"type":"function"},{"constant":false,"inputs":[{"name":"_to","type":"address"},{"name":"_value","type":"uint256"}],"name":"transfer","outputs":[{"name":"","type":"bool"}],"payable":false,"type":"function"},{"constant":true,"inputs":[],"name":"tokenCreationMin","outputs":[{"name":"","type":"uint256"}],"payable":false,"type":"function"},{"constant":true,"inputs":[],"name":"funding","outputs":[{"name":"","type":"bool"}],"payable":false,"type":"function"},{"constant":true,"inputs":[],"name":"tokenCreationRate","outputs":[{"name":"","type":"uint256"}],"payable":false,"type":"function"},{"constant":true,"inputs":[],"name":"fundingStartBlock","outputs":[{"name":"","type":"uint256"}],"payable":false,"type":"function"},{"constant":false,"inputs":[],"name":"create","outputs":[],"payable":true,"type":"function"},{"inputs":[{"name":"_golemFactory","type":"address"},{"name":"_migrationMaster","type":"address"},{"name":"_fundingStartBlock","type":"uint256"},{"name":"_fundingEndBlock","type":"uint256"}],"type":"constructor"},{"anonymous":false,"inputs":[{"indexed":true,"name":"_from","type":"address"},{"indexed":true,"name":"_to","type":"address"},{"indexed":false,"name":"_value","type":"uint256"}],"name":"Transfer","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"name":"_from","type":"address"},{"indexed":true,"name":"_to","type":"address"},{"indexed":false,"name":"_value","type":"uint256"}],"name":"Migrate","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"name":"_from","type":"address"},{"indexed":false,"name":"_value","type":"uint256"}],"name":"Refund","type":"event"}]',
        GNTB: '[{"constant":false,"inputs":[{"name":"payments","type":"bytes32[]"},{"name":"closureTime","type":"uint64"}],"name":"batchTransfer","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[],"name":"name","outputs":[{"name":"","type":"string"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"name":"_spender","type":"address"},{"name":"_value","type":"uint256"}],"name":"approve","outputs":[{"name":"","type":"bool"}],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[],"name":"totalSupply","outputs":[{"name":"","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"name":"_from","type":"address"},{"name":"_to","type":"address"},{"name":"_value","type":"uint256"}],"name":"transferFrom","outputs":[{"name":"","type":"bool"}],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":false,"inputs":[{"name":"_value","type":"uint256"}],"name":"withdraw","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[],"name":"decimals","outputs":[{"name":"","type":"uint8"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"name":"to","type":"address"},{"name":"value","type":"uint256"},{"name":"data","type":"bytes"}],"name":"transferAndCall","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":false,"inputs":[{"name":"_value","type":"uint256"}],"name":"burn","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":false,"inputs":[],"name":"transferFromGate","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":false,"inputs":[{"name":"_spender","type":"address"},{"name":"_subtractedValue","type":"uint256"}],"name":"decreaseApproval","outputs":[{"name":"","type":"bool"}],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[{"name":"_owner","type":"address"}],"name":"balanceOf","outputs":[{"name":"balance","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[],"name":"TOKEN","outputs":[{"name":"","type":"address"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[],"name":"openGate","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[],"name":"symbol","outputs":[{"name":"","type":"string"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"name":"_to","type":"address"},{"name":"_value","type":"uint256"}],"name":"transfer","outputs":[{"name":"","type":"bool"}],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":false,"inputs":[{"name":"_value","type":"uint256"},{"name":"_destination","type":"address"}],"name":"withdrawTo","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[{"name":"_user","type":"address"}],"name":"getGateAddress","outputs":[{"name":"","type":"address"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"name":"_spender","type":"address"},{"name":"_addedValue","type":"uint256"}],"name":"increaseApproval","outputs":[{"name":"","type":"bool"}],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[{"name":"_owner","type":"address"},{"name":"_spender","type":"address"}],"name":"allowance","outputs":[{"name":"","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"},{"inputs":[{"name":"_gntToken","type":"address"}],"payable":false,"stateMutability":"nonpayable","type":"constructor"},{"anonymous":false,"inputs":[{"indexed":true,"name":"from","type":"address"},{"indexed":true,"name":"to","type":"address"},{"indexed":false,"name":"value","type":"uint256"},{"indexed":false,"name":"closureTime","type":"uint64"}],"name":"BatchTransfer","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"name":"gate","type":"address"},{"indexed":true,"name":"user","type":"address"}],"name":"GateOpened","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"name":"to","type":"address"},{"indexed":false,"name":"amount","type":"uint256"}],"name":"Mint","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"name":"burner","type":"address"},{"indexed":false,"name":"value","type":"uint256"}],"name":"Burn","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"name":"owner","type":"address"},{"indexed":true,"name":"spender","type":"address"},{"indexed":false,"name":"value","type":"uint256"}],"name":"Approval","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"name":"from","type":"address"},{"indexed":true,"name":"to","type":"address"},{"indexed":false,"name":"value","type":"uint256"}],"name":"Transfer","type":"event"}]',
        GNTDeposit: '[{"constant":false,"inputs":[{"name":"_from","type":"address"},{"name":"_amount","type":"uint256"},{"name":"_subtask_id","type":"bytes32"}],"name":"reimburseForVerificationCosts","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[{"name":"","type":"address"}],"name":"balances","outputs":[{"name":"","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[],"name":"coldwallet","outputs":[{"name":"","type":"address"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[{"name":"_owner","type":"address"}],"name":"isUnlocked","outputs":[{"name":"","type":"bool"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[],"name":"concent","outputs":[{"name":"","type":"address"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[{"name":"_owner","type":"address"}],"name":"isLocked","outputs":[{"name":"","type":"bool"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"name":"_to","type":"address"}],"name":"withdraw","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[{"name":"_owner","type":"address"}],"name":"balanceOf","outputs":[{"name":"","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"name":"_whom","type":"address"},{"name":"_burn","type":"uint256"}],"name":"burn","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":false,"inputs":[],"name":"unlock","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[{"name":"_owner","type":"address"}],"name":"isTimeLocked","outputs":[{"name":"","type":"bool"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"name":"_requestor","type":"address"},{"name":"_provider","type":"address"},{"name":"_amount","type":"uint256"},{"name":"_subtask_id","type":"bytes32"}],"name":"reimburseForSubtask","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":false,"inputs":[{"name":"_from","type":"address"},{"name":"_amount","type":"uint256"},{"name":"","type":"bytes"}],"name":"onTokenReceived","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[{"name":"_owner","type":"address"}],"name":"getTimelock","outputs":[{"name":"","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[{"name":"","type":"address"}],"name":"locked_until","outputs":[{"name":"","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[],"name":"withdrawal_delay","outputs":[{"name":"","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[],"name":"lock","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[],"name":"token","outputs":[{"name":"","type":"address"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"name":"_requestor","type":"address"},{"name":"_provider","type":"address"},{"name":"_amount","type":"uint256"},{"name":"_closure_time","type":"uint256"}],"name":"reimburseForNoPayment","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"inputs":[{"name":"_token","type":"address"},{"name":"_concent","type":"address"},{"name":"_coldwallet","type":"address"},{"name":"_withdrawal_delay","type":"uint256"}],"payable":false,"stateMutability":"nonpayable","type":"constructor"},{"anonymous":false,"inputs":[{"indexed":true,"name":"_owner","type":"address"},{"indexed":false,"name":"_amount","type":"uint256"}],"name":"Deposit","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"name":"_from","type":"address"},{"indexed":true,"name":"_to","type":"address"},{"indexed":false,"name":"_amount","type":"uint256"}],"name":"Withdraw","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"name":"_owner","type":"address"}],"name":"Lock","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"name":"_owner","type":"address"}],"name":"Unlock","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"name":"_who","type":"address"},{"indexed":false,"name":"_amount","type":"uint256"}],"name":"Burn","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"name":"_requestor","type":"address"},{"indexed":true,"name":"_provider","type":"address"},{"indexed":false,"name":"_amount","type":"uint256"},{"indexed":false,"name":"_subtask_id","type":"bytes32"}],"name":"ReimburseForSubtask","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"name":"_requestor","type":"address"},{"indexed":true,"name":"_provider","type":"address"},{"indexed":false,"name":"_amount","type":"uint256"},{"indexed":false,"name":"_closure_time","type":"uint256"}],"name":"ReimburseForNoPayment","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"name":"_from","type":"address"},{"indexed":false,"name":"_amount","type":"uint256"},{"indexed":false,"name":"_subtask_id","type":"bytes32"}],"name":"ReimburseForVerificationCosts","type":"event"}]',
        Faucet: '[{"constant":false,"inputs":[],"name":"create","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[],"name":"token","outputs":[{"name":"","type":"address"}],"payable":false,"stateMutability":"view","type":"function"},{"inputs":[{"name":"_token","type":"address"}],"payable":false,"stateMutability":"nonpayable","type":"constructor"}]',
    };
    let CONTRACTS = { };

    CONTRACTS[RINKEBY] = {
        GNT: '0x924442A66cFd812308791872C4B242440c108E19',
        GNTB: '0x123438d379BAbD07134d1d4d7dFa0BCbd56ca3F3',
        Faucet: '0x77b6145E853dfA80E8755a4e824c4F510ac6692e',
    };

    CONTRACTS[MAINNET] = {
        GNT: '0xa74476443119A942dE498590Fe1f2454d7D4aC0d',
        GNTB: '0xA7dfb33234098c66FdE44907e918DAD70a3f211c',
    };


    if (typeof web3 !== 'undefined') {
        // Use Mist/MetaMask's provider
        web3js = new Web3(web3.currentProvider);
    } else {
        console.log('No web3? You should consider trying MetaMask!')
        // fallback - use your fallback strategy (local node / hosted node + in-dapp id mgmt / fail)
        web3js = new Web3(new Web3.providers.HttpProvider("http://localhost:8545"));
    }

    console.log('s1');

    let gueth = {
        getBalance: balance
    };


    try {
        gueth.gnt = web3js.eth.contract(JSON.parse(ABI.GNT)).at(CONTRACTS[web3js.version.network].GNT);
        gueth.gntb = web3js.eth.contract(JSON.parse(ABI.GNTB)).at(CONTRACTS[web3js.version.network].GNTB);
        if (CONTRACTS[web3js.version.network].Faucet) {
            gueth.faucet =  web3js.eth.contract(JSON.parse(ABI.Faucet)).at(CONTRACTS[web3js.version.network].Faucet);
        }

    }
    catch (e) {
        // TODO
    }

    function balance(token, address) {
        return new Promise((resolve, reject) => {
            if (typeof token === 'undefined') {
                return reject('No web3? You should consider trying MetaMask!')
            }
            token.decimals((error, decimals) => {
                if (error) {
                    return reject(error);
                }
                token.balanceOf(address, (error, balance) => {
                    if (error) return reject(error);

                    resolve(balance.div(10**decimals))
                })
            })
        });
    }

    function getGnt(address) {
        return balance(gueth.gnt, address)
    }

    let net = null;
    try {
        net = web3js.version.network;
    }
    catch (e) {
        net = '1';
    }

    console.log('s4');
    //balance(gueth.gnt, web3js.eth.accounts[0]).then(v => console.log('gnt=', v.toString()));
    //balance(gueth.gntb, web3js.eth.accounts[0]).then(v => console.log('gntb=', v.toString()));


    gueth.net = (net == RINKEBY ? "testnet" : "mainnet");

    gueth.accounts = function() {
        return web3js.eth.accounts;
    };

    gueth.checkAddress = (address) => web3js.isAddress(address);

   window.gueth = gueth;
});


