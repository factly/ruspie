import React from 'react'

type props = {
	color?: string
}

export function PlusIcon(props: props) {
	const { color = "#fff" } = props
	return (
		<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 20 20" fill="none">
			<path d="M10 4.1665V15.8332" stroke={color} strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" />
			<path d="M4.16699 10H15.8337" stroke={color} strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" />
		</svg>
	)
}

