import { ListGuides } from "./types";
import { invoke } from "@tauri-apps/api/tauri";

async function getAllGuides () {
	const res = await invoke<ListGuides>('get_guides')
	return res
}

export {getAllGuides}
