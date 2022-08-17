import * as nearAPI from 'near-api-js';

export interface NearUserView {
    accountId: string;
    balance: string;
}

export interface AppStatusMessageSetContext {
    message: string;
    account_id: string;
}
export interface AppStatusMessageGetContext {
    account_id: string;
}
export interface NearContractStatusMessageMethods {
    // see awesome-rust-dapp/src/lib.rs#StatusMessage.set_status
    set_status: (context: AppStatusMessageSetContext, boatloadOfGas: string) => Promise<void>;
    // see awesome-rust-dapp/src/lib.rs#StatusMessage.get_status
    get_status: (context: AppStatusMessageGetContext) => Promise<string>

}

export interface NearContractContext {
    contract: nearAPI.Contract & NearContractStatusMessageMethods | undefined;
    currentUser: NearUserView|undefined;
    nearConfig: any|undefined;
    wallet: nearAPI.WalletConnection|undefined;
}

export interface FormProps {
    onClickHello: (event: any) => void,
    onSubmit: (event: any) => void,
    currentUser?: NearUserView,
  }
  