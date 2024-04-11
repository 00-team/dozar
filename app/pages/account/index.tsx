import CountUp from '!/components/countUp'
import { WalletIcon } from '!/icons/account'
import { user } from '!/store/user'
import { Link, Outlet, useNavigate } from '@solidjs/router'
import { Component, onMount } from 'solid-js'

import './style/account.scss'

const Account: Component<{}> = props => {
    const navigate = useNavigate()

    let section = window.location.pathname.split('/')[2]

    onMount(() => {
        if (!user.token) return navigate('/login')
    })

    console.log(section)

    return (
        <main class='account-container'>
            <div class='account-wrapper'>
                <section class='account-header'>
                    <Link href='/account/wallet' class='wallet column'>
                        <WalletIcon />
                        <span class=' title_small'>کیف پول</span>
                    </Link>
                    <Link href='/account/auctions' class='auctions column'>
                        <p class='title_small'>مزایده های شرکت کرده</p>
                        <span class='title_small number'>
                            <CountUp addTime={50} end={19} steps={2} />
                        </span>
                    </Link>
                    <Link
                        href='/account/auctions-win'
                        class='auctions-win column'
                    >
                        <p class='title_small'>مزایده های برنده شده </p>
                        <span class='title_small number'>
                            <CountUp addTime={50} end={19} steps={1} />
                        </span>
                    </Link>
                    <div class='user_name column'>
                        <p class='title_small'>صدرا ادلاعی پور</p>
                        <Link
                            href='/account/profile'
                            class='title_smaller edit-btn'
                        >
                            ویرایش پروفایل
                        </Link>
                    </div>
                    <div class='user_img'>
                        <img src='https://picsum.photos/300/300' alt='' />
                    </div>
                </section>

                <section class='account-body'>
                    <Outlet />
                </section>
            </div>
        </main>
    )
}

export default Account
