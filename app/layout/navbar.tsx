import { AddToHomeIcon } from '!/icons/addHome'
import {
    AuctionIcon,
    CloseIcon,
    ContactusIcon,
    HomeIcon,
    MenuIcon,
} from '!/icons/navbar'
import { Link } from '@solidjs/router'
import { Component, createSignal, JSX } from 'solid-js'

import './style/navbar.scss'

const Navbar: Component<{}> = props => {
    return (
        <>
            <NavPc />
            <NavMobile />
        </>
    )
}

const NavMobile: Component = () => {
    const [openNav, setOpenNav] = createSignal(false)
    return (
        <>
            <nav class='nav-container-mobile'>
                <div class='nav-open' onclick={() => setOpenNav(true)}>
                    <MenuIcon size={35} />
                </div>
                <Link href='/' class='nav-logo hero_title'>
                    DOZAR
                </Link>
            </nav>
            <div class='nav-mobile-open' classList={{ active: openNav() }}>
                <div class='close-nav icon' onclick={() => setOpenNav(false)}>
                    <CloseIcon size={30} />
                </div>
                <div class='mobile-links'>
                    <NavLink
                        Icon={HomeIcon}
                        link='/'
                        title='صفحه اصلی'
                        onclick={() => setOpenNav(false)}
                    />
                    <NavLink
                        Icon={AuctionIcon}
                        link='/'
                        title='مزایده ها'
                        onclick={() => setOpenNav(false)}
                    />
                    <NavLink
                        Icon={ContactusIcon}
                        link='/'
                        title='ارتباط با ما'
                        onclick={() => setOpenNav(false)}
                    />
                    <NavLink
                        Icon={AddToHomeIcon}
                        link=''
                        title='اضافه به صفحه اصلی'
                        onclick={() => {
                            setOpenNav(false)
                            document.querySelector('.add-home').className +=
                                ' active'
                        }}
                    />
                </div>
            </div>
        </>
    )
}

const NavPc = () => {
    return (
        <nav class='nav-container-pc'>
            <div class='nav-links'>
                <NavLink Icon={HomeIcon} link='/' title='صفحه اصلی' />
                <NavLink
                    Icon={AuctionIcon}
                    link='/auctions'
                    title='مزایده ها'
                />
                <NavLink
                    Icon={ContactusIcon}
                    link='/contact'
                    title='ارتباط با ما'
                />
            </div>
            <Link href='/' class='nav-logo hero_title'>
                DOZAR
            </Link>
        </nav>
    )
}

interface NavLink {
    Icon: (P: any) => JSX.Element | JSX.Element
    title: string
    link: string
    onclick?: () => void
}
const NavLink: Component<NavLink> = ({ Icon, link, title, onclick }) => {
    return (
        <>
            {link !== '' ? (
                <Link
                    href={link}
                    class='nav-link title_small'
                    classList={{ active: link === window.location.pathname }}
                    onclick={onclick}
                >
                    <div class='holder icon'>
                        <Icon />
                    </div>
                    <p class='data'>{title}</p>
                </Link>
            ) : (
                <button
                    class='nav-link title_small'
                    classList={{ active: link === window.location.pathname }}
                    onclick={onclick}
                >
                    <div class='holder icon'>
                        <Icon />
                    </div>
                    <p class='data'>{title}</p>
                </button>
            )}
        </>
    )
}

export default Navbar
