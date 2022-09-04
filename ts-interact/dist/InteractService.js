import { DEFAULT_FUNCTION_CALL_GAS } from "near-api-js";
import BN from "bn.js";
class InteractService {
    constructor(near, account) {
        this.near = near;
        this.account = account;
    }
    async getBlock(id) {
        return await this.near.connection.provider.block({ blockId: id });
    }
    async getCurrentHeight() {
        const response = await this.near.connection.provider.block({
            finality: "final",
        });
        return response === null || response === void 0 ? void 0 : response.header.height;
    }
    async queryAccount(account_id) {
        return await this.near.connection.provider.query({
            request_type: "view_account",
            finality: "final",
            account_id: account_id,
        });
    }
    async viewContractState(contractId, methodName, args = {}) {
        return await this.account.viewFunction(contractId, methodName, args);
    }
    async changeContractState(contractId, methodName, args, gas = DEFAULT_FUNCTION_CALL_GAS, attachedDeposit = new BN(0), walletMeta = "", walletCallbackUrl = "", stringify = undefined, jsContract = false) {
        const payload = {
            contractId: contractId,
            methodName: methodName,
            args: args,
            gas: gas,
            attachedDeposit: attachedDeposit,
            walletMeta: walletMeta,
            walletCallbackUrl: walletCallbackUrl,
            stringify: stringify,
            jsContract: jsContract
        };
        return await this.account.functionCall(payload);
    }
}
export default InteractService;
//# sourceMappingURL=InteractService.js.map