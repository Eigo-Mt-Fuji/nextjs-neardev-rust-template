import * as nearAPI from 'near-api-js';

export interface NearUserView {
    accountId: string;
    balance: string;
}

// export interface AppStatusMessageSetContext {
//     message: string;
//     account_id: string;
// }
// export interface AppStatusMessageGetContext {
//     account_id: string;
// }
// export interface NearContractStatusMessageMethods {
//     // see awesome-rust-dapp/src/lib.rs#StatusMessage.set_status
//     set_status: (context: AppStatusMessageSetContext, boatloadOfGas: string) => Promise<void>;
//     // see awesome-rust-dapp/src/lib.rs#StatusMessage.get_status
//     get_status: (context: AppStatusMessageGetContext) => Promise<string>
// }
export interface NearCallFtBalanceOfContext {
    account_id: string;
}
export interface AppReportSteudyCommitContext {
    receiver_id: string;
    memo?: string;
    urls: string[];
}
export interface SteadyStudyTokenContractMethods {
    ft_report_study_commit: (context: AppReportSteudyCommitContext, boatloadOfGas: string) => Promise<void>;
    ft_balance_of: (context: NearCallFtBalanceOfContext) => Promise<string>;
}
export interface NearContractContext {
    contract: nearAPI.Contract & SteadyStudyTokenContractMethods | undefined;
    currentUser: NearUserView|undefined;
    nearConfig: any|undefined;
    wallet: nearAPI.WalletConnection|undefined;
}

export interface FormProps {
    onSubmit: (event: any) => void,
    currentUser: NearUserView,
}
