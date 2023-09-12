import React from 'react'
import { FeatureType } from '@/types/feature'
import SearchSql from './searchSql'
import Graphql from './graphql'
import RestApi from './restApi'
import Schema from './schema'

type Props = {
	feature: FeatureType
}


function Feature({ feature }: Props) {

	if(feature.value	=== "search"){
		return (<SearchSql />)
	}

	if(feature.value	=== "rest"){
		return (<RestApi />)
	}

	if(feature.value	=== "graphql"){
		return (<Graphql />)
	}

	if (feature.value === "schema") {
		return (<Schema />)
	}

	return (<h2>Feature</h2>)
}

export default Feature
