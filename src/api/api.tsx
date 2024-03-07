import { invoke } from "@tauri-apps/api";
import { ListGuides } from "./types";

async function getAllGuides () {
	const res = await invoke<ListGuides>('get_guides')
	return res
}

export {getAllGuides}
