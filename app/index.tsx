import { lazy, onMount } from 'solid-js'
import { render } from 'solid-js/web'

const Home = lazy(() => import('./pages/home'))
const AddHome = lazy(() => import('./components/addHome'))

import './style/base.scss'
import './style/config.scss'
import './style/font/imports.scss'
import './style/theme.scss'

const APP_ID = 'dozar-offical-pwa'

export const App = () => {
    onMount(() => {
        if ('serviceWorker' in navigator) {
            navigator.serviceWorker.register('/sw.js')
        }
    })

    const isSafari = navigator.userAgent.includes('Safari')

    return (
        <>
            {isSafari && // @ts-ignore
                !window.navigator.standalone && <AddHome />}

            <Home />

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
