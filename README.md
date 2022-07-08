# Sub-account Custom Script

This is a contract for CKB network and also it is a custom script for [das-contracts](https://github.com/dotbitHQ/das-contracts).
With this custom script user can define the price of sub-accounts, and elevate the security to contact level.


## Deployment Information

### Mainnet

TODO

### Testnet

Type Script:

```
{
    "code_hash": "0x00000000000000000000000000000000000000000000000000545950455f4944",
    "args": "0xf15f519ecb226cd763b2bcbcab093e63f89100c07ac0caebc032c788b187ec99",
    "hash_type": "type"
}
```

Custom Script Config Value:

```
0x01f15f519ecb226cd763b2bcbcab093e63f89100c07ac0caebc032c788b187ec99
```

Type ID:

```
0x4d224050b1de20454979d44c474e1e988e9eaa24cfd2319ab9bd4fdd439be4fe
```


## Configuration

This script depends on `SubAccountCell.data.script_args` to store a hash of real configuration which means you need to set the script_args 
in the transaction named `config_sub_account_custom_script action`. And transaction builder who want to create or renew sub-account needs 
put the configuration in `transaction.witnesses[]`.

The configuration consists of header, version and body：

```
[header][version][body]
```

- header, 10 bytes of fixed content `0x7363726970742D303031` which means `script-001`, custom script use the header to find the witness able to be parsed.
- version, 4 bytes litter-endian u32 number, it indicates the version of the `body` field which allow developer upgrading the structure of the `body` field.
- body, a [molecule structure](https://github.com/nervosnetwork/molecule) with variable length, the type is `table PriceConfigList` which come from [das-types](https://github.com/dotbitHQ/das-types/blob/master/schemas/cell.mol)；


## Building

First you need to install [capsule](https://github.com/nervosnetwork/capsule) and [Docker](https://www.docker.com/), capsule will use docker 
to compile binaries because the compiling environment is really complex. 

Then you may build the contract with targets in Makefile, for example:

- `make` will build the developing version of contract;
- `make release` will build the releasing version of contract;

> Because of the complexity and the limitation of function is hard to overcome for beginners, we do not implement any unit tests in this 
sample project. You may create a template project with capsule instantly and explore how to write unit tests yourself.


## Deployment

To deploy the contract you need prepare a lot of things:

- A CKB full node which will allow ckb-cli to build index for live cells, initializing a full node will cost you about two days;
- A ckb-cli binary with fully synchronized index, this will also cost you about half day;
- A CKB address with enough CKBytes;

After all the above is ready, you need to edit the value of `--address` in the Makefile to your address. At the last you may safely run 
`make deploy-testnet` or `make deploy-mainnet`.

> DO NOT forget commit and push the json files generated in `migrations/` directory, without this information you will not be able to update
> the contract with capsule in the future. If that already happened, you need to construct the transaction of updating contract manually.
