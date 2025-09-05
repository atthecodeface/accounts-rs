//a To do
// Orbit, more names

//a Imports
import {Log} from "./log.js";
import * as utils from "./utils.js";
import * as html from "./html.js";
import {tabbed_configure} from "./tabbed.js";
import {ServerRequests} from "./server_requests.js";

//a Accounts
class Accounts {
    constructor(server_requests, div_id) {
        this.server_requests = new ServerRequests("", 4);
        this.account_db_ids = [];
        this.div = document.getElementById(div_id);
        this.add_html_elements();
        this.get_accounts();
    }
    
    //mp add_html_elements
    add_html_elements() {
        this.fill_accounts();
    }

    //mp fill_accounts
    fill_accounts() {
        html.clear(this.div);
        const headings = [];
        const contents = [];
        headings.push("Account id", "Org", "Name", "Account");
        for (const acct_data of this.account_db_ids) {
            const db_id = acct_data[0];
            const data = acct_data[1];
            if (data.length >= 3) {
                contents.push([`${db_id}`, `${data[0]}`, `${data[1]}`, `${data[2]}`]);
            }
        }
        const table = html.table("", headings, contents);
        this.div.appendChild(table);
    }

    //mp get_accounts
    get_accounts() {
        this.server_requests.add_request("fetch accounts list", "accounts","list",
                                         this.set_accounts.bind(this),
                                        );
    }
    
    //mp set_accounts
    set_accounts(data) {
        this.account_db_ids = [];
        for (const db_item of data) {
            const db_id = db_item[0];
            const account = db_item[1];

            this.account_db_ids.push( [ db_id, [account.org, account.name, `${account.desc.Uk.sort_code} : ${account.desc.Uk.account}`]]);
        }
        this.fill_accounts();
    }

}

//a RustAccounts
//c RustAccounts
class RustAccounts {
    //cp constructor
    constructor(params) {
        this.server_requests = new ServerRequests("", 4);
        this.accounts = new Accounts(this.server_requests, "accounts");
    }

    //mp tab_selected
    tab_selected(tab_id) {
        const e = document.getElementById("controls");
        if (!e) {
            return;
        }
    }

}

//a Top level on load...
window.rust_accounts = null;
function complete_init() {
    const location_url = new URL(location);
    // window.log = new Log(document.getElementById("Log"));
    window.rust_accounts = new RustAccounts(location_url.searchParams);
}

window.addEventListener("load", (e) => {
    complete_init();
    tabbed_configure("#tab-list", 
                     (id) => {if (window.rust_accounts) {window.rust_accounts.tab_selected(id);}});
});
