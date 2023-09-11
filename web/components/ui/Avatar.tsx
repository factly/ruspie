import Image from 'next/image';
import React from 'react'
import { FC } from 'react';
export interface StaticImageData {
	src: string;
	height: number;
	width: number;
	blurDataURL?: string;
	blurWidth?: number;
	blurHeight?: number;
}

export interface AvatarProps {
	src: string | StaticImageData;
	alt: string;
}

export const Avatar: FC<AvatarProps> = ({ src, alt }) => {
	return (
		<Image
			className="w-10 h-10 rounded-full bg-white" src={src} alt={alt}
			width={40}
			height={40}
		/>
	)
}

export default Avatar
