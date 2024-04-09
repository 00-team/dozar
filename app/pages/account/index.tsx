import { user } from '!/store/user'
import { useNavigate } from '@solidjs/router'
import { Component, onMount } from 'solid-js'

import './style/account.scss'

const Account: Component<{}> = props => {
    const navigate = useNavigate()

    onMount(() => {
        if (!user.token) return navigate('/login')
    })

    return <main class='account-container'></main>
}

export default Account
