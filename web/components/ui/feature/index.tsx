import React from 'react'
import { FeatureType } from '@/types/feature'
import SearchSql from './searchSql'
import Graphql from './graphql'

type Props = {
	feature: FeatureType
}


function Feature({ feature }: Props) {

	if(feature.value	=== "search"){
		return (<SearchSql />)
	}

	if(feature.value	=== "rest"){
		return (<h2>Rest</h2>)
	}

	if(feature.value	=== "graphql"){
		return (<Graphql />)
	}

	return (<h2>Feature</h2>)
}

export default Feature
