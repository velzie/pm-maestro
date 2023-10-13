import { createEventDispatcher } from "svelte";

export async function sendkillsignal(id: number, signal: number) {
    await fetch(`/api/${id}/kill`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: signal.toString(),
    });

}
export async function startprocess(id: number) {
    await fetch(`/api/${id}/start`, {
        method: "POST",
    });

}
export async function deleteProcess(id: number) {
    await fetch(`/api/${id}`, {
        method: "DELETE",
    });

}