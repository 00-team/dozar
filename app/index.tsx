import { Route, Router, Routes } from '@solidjs/router'
import { lazy } from 'solid-js'
import { render } from 'solid-js/web'

const Home = lazy(() => import('./pages/home'))

render(
    () => (
        <>
            <Router>
                <Routes>
                    <Route path='/' component={Home}></Route>
                </Routes>
            </Router>
        </>
    ),
    document.getElementById('root')
)
