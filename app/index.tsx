import { Route, Router, Routes } from '@solidjs/router'
import { lazy, onCleanup, onMount } from 'solid-js'
import { render } from 'solid-js/web'
import { getDB } from './db'
import { SHOW_ADDHOME_AGAIN } from './store/dbLabels'

const Home = lazy(() => import('./pages/home'))
const Navbar = lazy(() => import('./layout/Navbar'))
const Auctions = lazy(() => import('./pages/auctions'))
const Auction = lazy(() => import('./pages/auction'))

const Account = lazy(() => import('./pages/account'))
const Login = lazy(() => import('./pages/login'))

const AddHome = lazy(() => import('./components/addHome'))

import './style/base.scss'
import './style/config.scss'
import './style/font/imports.scss'
import './style/theme.scss'

const APP_ID = 'dozar-offical-pwa'

export const App = () => {
    let addHomeElem: null | HTMLElement
    let timeout

    onMount(async () => {
        let showAddHomeAgain = await getDB(SHOW_ADDHOME_AGAIN)

        if (showAddHomeAgain === undefined) {
            timeout = setTimeout(() => {
                addHomeElem = document.querySelector('.add-home')

                if (addHomeElem.className.includes('active')) return

                addHomeElem.className += ' active'
            }, 20000)
        }
    })
    onCleanup(() => {
        clearTimeout(timeout)
    })

    onMount(() => {
        if ('serviceWorker' in navigator) {
            navigator.serviceWorker.register('/sw.js')
        }
    })

    const isSafari = navigator.userAgent.includes('Safari') && innerWidth <= 768

    return (
        <>
            {isSafari && // @ts-ignore
                !window.navigator.standalone && <AddHome />}

            <Router>
                <Navbar />
                <Routes>
                    <Route path={'/'} component={Home} />
                    <Route path={'/auctions'} component={Auctions} />
                    <Route path={'/auction/:slug'} component={Auction} />

                    <Route path={'/account'} component={Account} />
                    <Route path={'/login'} component={Login} />
                </Routes>
            </Router>
        </>
    )
}

render(() => <App />, document.getElementById('root'))
