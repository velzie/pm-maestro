import { createEventDispatcher } from "svelte";
import type { Process } from "./types";


let snackbar;
export function setsnackbar(bar) {
    snackbar = bar;
}

export async function sendkillsignal(id: number, signal: number) {
    await dfetch(`/api/${id}/kill`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: signal.toString(),
    });

}

export async function deleteProcess(id: number) {
    await dfetch(`/api/${id}`, {
        method: "DELETE",
    });

}

export async function dfetch(url: string, opts: any) {
    let r = await fetch(url, opts);
    if (r.status != 200) {
        snackbar({
            message: await r.text(),
            closable: true,
        })
    }
    return r;
}

export async function patchprocess(id: number, name: string, command: string, user: string, dir: string) {
    return await dfetch(`/api/${id}`, {
        method: "PATCH",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            name,
            command,
            dir,
            user,
        }),
    });
}
