import {Account, DEFAULT_FUNCTION_CALL_GAS, Near} from "near-api-js";
import {AccountView, BlockId, BlockResult} from "near-api-js/lib/providers/provider";
import {ChangeFunctionCallOptions} from "near-api-js/lib/account";
import {FinalExecutionOutcome} from "near-api-js/lib/providers";
import BN from "bn.js";

class InteractService {
    readonly near: Near;
    readonly account: Account;

    constructor(near: Near, account: Account) {
        this.near = near;
        this.account = account
    }

    async getBlock(id: BlockId): Promise<BlockResult> {
        return await this.near.connection.provider.block({blockId: id});
    }

    async getCurrentHeight(): Promise<number> {
        const response: BlockResult = await this.near.connection.provider.block({
            finality: "final",
        });

        return response?.header.height;
    }

    async queryAccount(account_id: string): Promise<AccountView> {
        return await this.near.connection.provider.query({
            request_type: "view_account",
            finality: "final",
            account_id: account_id,
        });
    }

    async viewContractState(contractId: string, methodName: string, args: any = {}): Promise<any> {
        return await this.account.viewFunction(contractId, methodName, args);
    }

    async changeContractState(contractId,
                              methodName,
                              args,
                              gas = DEFAULT_FUNCTION_CALL_GAS,
                              attachedDeposit = new BN(0),
                              walletMeta = "",
                              walletCallbackUrl = "",
                              stringify = undefined,
                              jsContract = false
    ): Promise<FinalExecutionOutcome> {

        const payload: ChangeFunctionCallOptions = {
            contractId: contractId,
            methodName: methodName,
            args: args,
            gas: gas,
            attachedDeposit: attachedDeposit,
            walletMeta:walletMeta,
            walletCallbackUrl:walletCallbackUrl,
            stringify:stringify,
            jsContract:jsContract
        }

        return await this.account.functionCall(payload)
    }
}

export default InteractService
