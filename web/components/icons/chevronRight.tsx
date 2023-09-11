import React from 'react'

type Props = {
	size?: "small" | "large"
}

export function ChevronRightIcon({ size = "small" }) {

	if (size === "large") {
		return (
			<svg xmlns="http://www.w3.org/2000/svg" width="21" height="22" viewBox="0 0 21 22" fill="none">
				<g clip-path="url(#clip0_1_37227)">
					<path d="M14 11.25L8.75 6L7.51625 7.23375L11.5238 11.25L7.51625 15.2663L8.75 16.5L14 11.25Z" fill="#323232" />
				</g>
				<defs>
					<clipPath id="clip0_1_37227">
						<rect width="21" height="21" fill="white" transform="translate(21 0.75) rotate(90)" />
					</clipPath>
				</defs>
			</svg>
		)
	}
	return (
		<svg xmlns="http://www.w3.org/2000/svg" width="15" height="15" viewBox="0 0 15 15" fill="none">
			<path d="M5.61914 10.8579L9.11914 7.35791L5.61914 3.85791" stroke="#666666" strokeLinecap="round" strokeLinejoin="round" />
		</svg>
	)
}
