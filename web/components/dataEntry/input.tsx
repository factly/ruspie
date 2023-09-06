'use client';
import React, { InputHTMLAttributes } from 'react';

interface InputProps extends Omit<InputHTMLAttributes<HTMLInputElement>, 'prefix'> {
  prefix?: React.ReactNode | string;
}

const Input = React.forwardRef<HTMLInputElement, InputProps>(
	({ className, type, prefix, ...props }, ref) => {


		return (
			<div className="relative">
				{
					prefix && (
						<div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
							{prefix}
						</div>
					)
				}
				<input
					type={type}
					className={
						`flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50  ${prefix ? "pl-10" : ""} ${className}`
					}
					ref={ref}
					{...props}
				/>
			</div>
		)
	}
)
Input.displayName = "Input"

export { Input }
