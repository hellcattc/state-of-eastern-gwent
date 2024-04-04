type ListGuide = {
	id: number,
	name: string,
	created: string
}

type ListGuides = {
	guides: ListGuide[] 
}

export type {ListGuide, ListGuides}
