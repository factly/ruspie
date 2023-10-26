import React from 'react'
import EmptyIllustrationn from '@/assets/Empty.svg'
import Image from 'next/image'

export function NotFound() {
	return (
		<Image src={EmptyIllustrationn} alt="Not Found"
			width={400}
			height={400}
		/>
	)
}


