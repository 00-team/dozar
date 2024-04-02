import { Route, Router, Routes } from '@solidjs/router'
import { lazy, onCleanup, onMount } from 'solid-js'
import { render } from 'solid-js/web'
import { getDB } from './db'
import { SHOW_ADDHOME_AGAIN } from './store/dbLabels'

const Home = lazy(() => import('./pages/home'))
const MobileHome = lazy(() => import('./pages/mobile'))
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
            console.log('slm')
            timeout = setTimeout(() => {
                addHomeElem = document.querySelector('.add-home')

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
                <Routes>
                    <Route path={'/'} component={Home} />
                    <Route path={'/mobile'} component={MobileHome} />
                </Routes>
            </Router>

            {/* <Show when={user.token} fallback={<Login />}>
                {activeTab() === 'home' && <Home />}
                {activeTab() === 'map' && <Map />}
                {activeTab() === 'account' && <Account />}

                <Navbar />
            </Show> */}
        </>
    )
}

render(() => <App />, document.getElementById('root'))
