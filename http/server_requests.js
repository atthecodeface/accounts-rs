//a Imports
import {Log} from "./log.js";
import * as utils from "./utils.js";
import * as html from "./html.js";

//a ServerRequests
//c ServerRequests
export class ServerRequests {
    //cp Constructor
    constructor (uri, max_outstanding) {
        this.uri = uri;
        this.max_outstanding = max_outstanding;
        this.pending_requests = [];
        this.outstanding_requests = [];
    }

    //mp add_request
    add_request(reason, cmd, args, callback) {
        this.pending_requests.push([reason, cmd, args, callback]);
        this.try_to_request();
    }

    //mp try_to_request
    try_to_request() {
        while (true) {
            if (this.pending_requests.length == 0) {
                return;
            }
            if (this.outstanding_requests.length >= this.max_outstanding) {
                return;
            }

            const request = this.pending_requests.shift();
            const reason = request[0];
            const cmd = request[1];
            const args = request[2];
            const callback = request[3];
            const me = this;

            this.issue_fetch(reason, cmd, args, 
                                    (c) => me.completed_callback(reason, callback, c),
                                    this.fetch_err.bind(this));
        }
    }

    //mp completed_callback
    completed_callback(reason, callback, data) {
        callback(data);
        this.try_to_request();
    }

    //mp fetch_err
    fetch_err(error, reason) {
        console.log(`Failed to fetch from database: ${reason}: ${error}`);
    }


    //mp go_fetch
    async go_fetch(cmd, args) {
        let uri_args = args;
        if (utils.is_array(args)) {
            uri_args = "";
            let sep  = "";
            for (const arg of args) {
                if (utils.is_array(arg)) {
                    for (let i=1; i<arg.length; i++) {
                        uri_arg += sep + arg[0] + "=" + arg[i];
                        sep = "&";
                    }
                } else {
                    uri_args += sep + arg;
                    sep = "&";
                }
            }
        }
        const uri = `${this.uri}/exec_cmd/${cmd}?${uri_args}`;
        console.log(`Issuing fetch(${uri}`);
        return fetch(encodeURI(uri))
            .then((response) => {
                if (!response.ok) {
                    throw new Error(`Failed to fetch: ${response.status}`);
                }
                return response.json();
            });
    }

    //mp issue_fetch
    issue_fetch(reason, cmd, args, ok_callback, err_callback) {
        const me = this;
        let promises = [];
        promises.push(
            this.go_fetch(cmd, args)
                .then((m) => {
                    if (utils.is_array(m) && m.length>1 && m[0]=="ok") {
                        ok_callback(m[1]);
                    } else {
                        err_callback(reason, m);
                    }
                })
                .catch((err) => {
                    console.error(`Fetch problem: ${err.message}`);
                    err_callback(reason, err)
                }));
        Promise.all(promises).then(() => {
            console.log("Completed fetch");
        });
    }

}

