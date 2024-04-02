import { AddToHomeIcon, ShareIcon } from '!/icons/addHome'
import { Component } from 'solid-js'

import './style/addhome.scss'

interface AddhomeProps {
    deferredPrompt?: any
}

const AddHome: Component<AddhomeProps> = P => {
    window.addEventListener('appinstalled', () => {
        location.reload()
    })

    return (
        <div class='add-home'>
            <header>
                <object
                    data='/dozar.svg'
                    aria-labelledby='logo'
                    type=''
                ></object>
                <h1 class='title'>
                    وب اپلیکیشن دوزار را به صفحه اصلی موبایل خود اضافه کنید.
                </h1>
            </header>
            <div class='details title_small'>
                {P.deferredPrompt ? (
                    <>
                        <div class='detail'>
                            <div class='icon'>
                                <ShareIcon />
                            </div>
                            1. پایین صفحه روی دکمه نصب بزنید.
                        </div>

                        <div class='detail'>
                            <div class='icon'>
                                <AddToHomeIcon />
                            </div>
                            2. برنامه نصب شده و میتوانید در صفحه اصلی از ان
                            استفاده کنید.
                        </div>
                    </>
                ) : (
                    <>
                        <div class='detail'>
                            <div class='icon'>
                                <ShareIcon />
                            </div>
                            1. در نوار پایین روی دکمه Share بزنید.
                        </div>

                        <div class='detail'>
                            <div class='icon'>
                                <AddToHomeIcon />
                            </div>
                            2. در منوی باز شده در قسمت پایین ، گزینه Add to Home
                            Screen را انتخاب کنید.
                        </div>

                        <div class='detail'>
                            <div class='icon add'>Add</div>
                            3. در مرحله آخر در قسمت بالا روی Add بزنید.{' '}
                        </div>
                    </>
                )}
            </div>
            {!P.deferredPrompt ? (
                <button
                    class='basic-button got-it title'
                    onclick={() => {
                        // @ts-ignore
                        document.querySelector('.add-home').style.display =
                            'none'
                    }}
                >
                    متوجه شدم
                </button>
            ) : (
                <button
                    class='basic-button got-it title'
                    onclick={async () => {
                        P.deferredPrompt().prompt()

                        P.deferredPrompt().userChoice.then(resp => {
                            // If the user dismissed the prompt, clear the installSource
                            console.log('INSTALL_PROMPT_RESPONSE:', resp)
                        })
                    }}
                >
                    نصب
                </button>
            )}
        </div>
    )
}

export default AddHome
