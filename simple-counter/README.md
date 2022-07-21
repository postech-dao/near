# Near CLI 커맨드 정리

NEAR CLI는 NEAR Javascript API를 사용하기에, 이 곳에 없는 기능은 near-api-js 에서 찾아 사용해야합니다.

Deploy 과정에 필요한 Create Account, deploy 에 관한 command를 정리했습니다.

각 command에 대해서
 - argument : command에 필요한 변수들 
 - options : command 조건 설정 옵션
 - command (or with command example) : 커맨드 (및 사용 예시). argument와 동일한 text에 argument에 해당하는 값을 PDAO 상황에 맞춰서 변수만 고쳐주면 됩니다.
 - details : 커맨드에 대한 간략한 설명
 - comment : 커맨드에 대한 script에서 사용 관련 의견

으로 정리를 하였으니, 이상한 점이나 잘못된 정보가 확인되면 수정바랍니다.
(Validator에 대한 CLI는 현재 필요하지 않아 보여 제외하였습니다.)

simple counter build & deploy 시 사용할 것으로 보이는 CLI는 아래와 같습니다.
 - [near login](https://github.com/postech-dao/near/new/basten1209/simple-counter#near-login)
 - [near create-account](https://github.com/postech-dao/near/new/basten1209/simple-counter#near-create-account)
 - [near deploy](https://github.com/postech-dao/near/new/basten1209/simple-counter#near-deploy)
 - [near dev-deploy](https://github.com/postech-dao/near/new/basten1209/simple-counter#near-dev-deploy)
 - [near view](https://github.com/postech-dao/near/new/basten1209/simple-counter#near-view)
 - [near tx-status](https://github.com/postech-dao/near/new/basten1209/simple-counter#near-tx-status)

"deploy" and "dev-deploy" command 가 가장 중요한데 쓰임이 조금 달라 이에 관한 [설명](https://github.com/postech-dao/near/new/basten1209/simple-counter#additional-comment-about-deploy-and-dev-deploy-command)
을 부가적으로 작성하였습니다.


## Commands about Create Account 

### near login
 - arguments : none
 - options : default
 - command example
```
  near login
```
 - details 
	- near login 커맨드 입력 시, 웹페이지로 넘어가며 Near Wallet full access를 요청합니다.
	- access  허용 시 .near-credentials 라는 이름의 hidden directory가 형성되며 Access Key가 저장됩니다.
	- hidden directory 내부에는 account_id, public_key, private_key가 저장됩니다.
 - comment
  - deploy할 때 사용할 계좌 생성에 사용해야 합니다. 다만 near login을 수행 시 웹으로 넘어가 작업을 하는 부분이 있는데, 좀 애매해서 script에 사용할 수 있는 다른 command를 찾아봐야 할 듯 합니다.

### near keys
 - argument : accountId
 - options : default
 - command
```
  near keys accountId
```
 - details
	 - 주어진 account에 all access keys를 display합니다.
 - comment
  - account key 확인하는 command라 배포 스크립트에 사용하지 않을 듯 합니다.

### near generate-key

 - argument : accountId, none
 - options : --useLedgerKey, --seedPhrase, --seedPath
 - command case1
```
  near generate-key
```
 - details
	 - .near-credentials 내의 key에 해당하는 pair를 생성합니다.
 - command case2
```
  near generate-key accountId
```
 - details
	 - accountId에 해당하는 .near-credentials 내 key에 pair를 생성합니다.(accountId를 생성하는 것이 아님.)
 - command case3
```
  near generate-key --useLedgerKey(="HD path")
```
- details
	 - Ledger를 사용하여 사용하고 있는 HD path(괄호 부분 생략 시 default HD pair)에 대한 public key와 implicit account를 표시합니다.
 - command case4
```
  near generate-key (accountId) --seedPhrase="seed phrase"
```
- details
	 - seed phrase 를 사용하여 public key와 implicit한 account를 표시
 - comment
  - near generate-key는 특정 key가 포함된 account로 deploy를 하는 case에 사용하나 그 외에 options를 활용하는 경우에는 display와 관련된 부분이라 사용할 일이 없어보입니다.

### near add-key
 - argument : accountId, publicKey
 - option : none
 - command
```
  near add-key accountId publicKey
```
 - details
	 - 이미 존재하는 key를 account에 추가할 때 사용합니다.
	 - 새로운 key 추가는 near login 으로 합니다.
	 - full access key를 이렇게 만듭니다.
	 -  function access의 경우 다른 options이 필요하나, 쓸 일 없을 듯 합니다.
- comment
  - 현재 상황에서 사용하지 않을 듯 합니다.

### near delete-key
 - argument : accountId, publicKey
 - options : none
 - command
```
  near delete-key accountId publicKey
```
 - details
	 - 계좌에 이미 존재하는 key를 삭제합니다.
 - comment
  - 현재 상황에서 사용하지 않을 듯 합니다.

### near create-account
 - argument : accountId, --masterAccount
 - options : --initialBalance
 - command case1
```
  near create-account accountId --masterAccount masteraccount
```
 - command example
```
  near create-account 7e094afcfc4eda8a970f6648cdf0dbd6de --masterAccount example-acct.testnet
```
 - details
	 - account 개설과 initial balance 지불을 위한 masterAccount 계좌를 생성

 - command case2
```
  near create-account accountId --masterAccount masteraccount --initialBalance initialbalance
```
 - command example
```
  near create-account sub-acct2.example-acct.testnet --masterAccount example-acct.testnet --initialBalance 10
```
- details
 	- 초기 balance를 설정하여 masterAccount를 생성
 - comment
  - 이미 존재하는 account를 masterAccount(account를 생성하고 initial balance를 자유롭게 지불할 수 있는 account)하기 위한 것으로 보이는데,
우리가 원하는 것에 필요한 것인지는 모르겠습니다.
  - 다만 설명에 같이
If you are looking to create a top-level .testnet or .near account you can do so using "[near-api-js](https://docs.near.org/tools/near-api-js/cookbook#create-account)".

  - 이 부분에 제시된 코드를 통하여 account 개설을 하면 deploy에 문제가 없을 듯 합니다.

### near state
 - arguments : accountId
 - options : none
 - command
```
near state accountId
```
 - details : account의 state를 보여줍니다.
 - comment : deploy 과정에서 deploy가 제대로 이뤄졌는지 확인을 위해 사용합니다.

### near send
 - arguments : senderId, receiverId, amount
 - options : none
 - command
```
near send senderId receiverId amount
```
- details : senderId에서 receiverId로 amount 만큼의 NEAR Token을 보냅니다.
	 senderId는 near login을 통해 만들어 full access가 가능한 key를 가지고 있어야 합니다.
  - comment : simple counter 올릴 때는 필요없으나 다른 기능들 추가되면서 사용할 커맨드로 보입니다.

### near delete
 - arguments : accountId, beneficiaryId
 - options : none
 - command
```
near delete accountId beneficiaryId
```
 - details
  - accountId에 남은 잔액(Tokens)을 beneficiaryId로 옮기고 accounId를 삭제합니다.
 - comment
  - simple counter 올릴 때는 필요없으나 다른 기능들 추가되면서 사용할 커맨드로 보입니다.
 	- beneficiaryId에 문제가 있을 경우 Token이 자동적으로 burn 되므로, 이 커맨드를 사용할 때 beneficiaryId의 존재여부를 파악하는 command를 사용하여 case에 따라 이 command를 실행하도록 하면 token burn이 일어나는 불상사를 막을 듯 합니다.

## About Contracts ( Deploy )

### near deploy
 - argument : accountId, .wasmFile
 - options : initFunction, initArgs, initGas, initDeposit
 - command case1
```
near deploy --accountId accountId --wasmFile .wasmFile
```
 - details
	 - 주어진 accountId에 smart contract를 deploy합니다.
 - command case2
```
near deploy --accountId accountId --wasmFile .wasmFile --initFunction new --initArgs '{" ~~~ ": " ~~~~~"}' 
```
- details
	 - 주어진 accountId에 arguments나 function 등을 initialize 하여 deploy합니다.
 - comment
  - deploy에 관한 commend입니다. 아래 작성된 dev-deploy와 상황에 따라 이용하면 될 듯 합니다. 자세한 comment는 dev-deploy 밑에 작성하였습니다.

### near dev-deploy
 - argument : .wasmFile
 - options : none
 - command
```
near dev-deploy .wasmFile
```
 - details
	 - testnet에서만 사용가능한 command입니다.
	 - access key 없이 development account를 생성하고, smart contract에 deploy합니다.
 - comment
	 - 계정 생성, 삭제 및 재생성에 소요되는 시간을 줄이기 위해 testnet에서 deploy할 시 개발중에 사용합니다.
	 - near dev-deploy는 smart contract를 위한 임시 dev 계정(full access) 을 만들어 배포에 사용합니다. 다만 계정상태는 변경되지 않습니다.
	 - contract code나 state의 불일치 문제를 막기 위해 dev-deploy를 사용합니다.
 	 - 제가 일전에 issue를 통해 조사한 부분과 관련이 있어보이는데(issue #18, https://github.com/postech-dao/near/issues/18),
	 - dev-deploy 커맨드를 사용할 때는 code와 state가 bind되어있다고 생각하고 deploy를 하면 되는 것으로 보이고, 
	 - deploy 커맨드를 사용할 때는 code와 state가 분리되어있다고 생각하고 deploy를 진행해야 합니다.

## additional comment about "deploy" and "dev-deploy" command

- deploy command를 사용해야 하는 case
  - Mainnet을 이용하는 경우
  - testnet을 사용하나, 특정 account에 대해 deploy하는 경우

- dev-deploy command를 사용해야하는 case
  - testnet을 사용하며, 특정 account가 지정되지 않고 contract code의 test를 위해 deploy 하는 경우

- simple counter 등 특정 contract를 deploy 할 때는 dev-deploy를 사용하고, 여러 기능들 구현하고 다 되갈 때부터 deploy를 사용하면 될 듯 합니다.
- https://stackoverflow.com/questions/69537652/difference-in-use-cases-of-near-deploy-vs-near-dev-deploy 를 참고하였습니다.
 
### near call
 - argument : contractName, method_name, {args}, --accountId
 - options : --gas, --deposit, --amount, --depositYocto, --base64
 - command
```
near call contractName method_name '{args}' --account-id accountId
```
 - details
	 - function call 확인을 위해 사용하는 command입니다.
	 - state modify, 혹은 view가 가능한 contract call을 합니다.
	 - empty arguments를 사용 시 '{args}'에 '{}'만 입력하면 됩니다.
	 - call의 경우 gas를 사용하기 때문에, near login을 통해 생성한 accountId가 필요로 합니다.
 - comment
  - deploy 과정에서는 near call보다는 near view를 통해 deploy가 제대로 이뤄졌는지 확인하는 것이 좋을 것 같습니다.

### near view
 - argument : contractName, method_name, {args}
 - options : none
 - command
```
near view contractName method_name '{args}'
```
- details
	 - empty arguments를 사용 시 '{args}'에 '{}'만 입력하면 됩니다.
- comment
  - view는 gas fee가 나가지 않아서, modify 하는 것이 아니라면 near call 보다는 near view command를 사용하는 것이 좋을 것 같습니다.

### near view-state
 - argument : accountId, finality or block-id
 - options : none
 - command case1
```
near view-state accountId --finality final
``` 
or
```
near view-state accountId --block-id block-id
```
 - details
	 - contract state를 utf-8 or borsh serialized format으로 리턴합니다.
 - comment
  - deploy script에 사용하지 않을 것 같습니다.

## About Transactions

### near tx-status
 - arguments : tx hash, --accountId or accountId:tx_hash
 - options : none
 - command
```
near tx-status accountId:tx hash
```
or
```
near tx-status tx hash --accountId accountId
```
 - details
  - accountId와 hash에 해당하는 tracsaction들을 display합니다.
 - comment
  - deploy 이후 제대로 deploy가 수행되었는지 확인을 위해 사용할 수 있습니다.


