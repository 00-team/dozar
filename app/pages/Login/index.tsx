import {
    CloseFillIcon,
    GoBackIcon,
    PhoneIcon,
    WarnnigIcon,
} from '!/icons/login'
import {
    Accessor,
    Component,
    createEffect,
    createSignal,
    onMount,
    Setter,
} from 'solid-js'
import { createStore, SetStoreFunction } from 'solid-js/store'

import './style/login.scss'

let phoneRegex = /^(0|09|09[0-9]{1,9})$/

const Login: Component<{}> = props => {
    const [stage, setStage] = createSignal<'phone' | 'code'>('phone')

    const [data, setData] = createStore({
        phone: '',
        code: '',
    })
    const [error, setError] = createSignal('')

    const [loading, setLoading] = createSignal(false)

    return (
        <main class='login-container'>
            <object data='/public/dozar.svg' aria-labelledby='logo'></object>

            <div class='login-wrapper'>
                <div class='login-data'>
                    <PhoneStage
                        data={data}
                        error={error}
                        setData={setData}
                        setError={setError}
                        setStage={setStage}
                        stage={stage}
                        loading={loading}
                        setLoading={setLoading}
                    />

                    <CodeStage
                        data={data}
                        error={error}
                        setData={setData}
                        setError={setError}
                        setStage={setStage}
                        stage={stage}
                        loading={loading}
                        setLoading={setLoading}
                    />
                </div>
            </div>
        </main>
    )
}

interface LoginChildProps {
    stage: Accessor<'phone' | 'code'>
    setStage: Setter<'phone' | 'code'>

    data: {
        phone: string
        code: string
    }
    setData: SetStoreFunction<{
        phone: string
        code: string
    }>

    error: Accessor<string>
    setError: Setter<string>

    loading: Accessor<boolean>
    setLoading: Setter<boolean>
}

const PhoneStage: Component<LoginChildProps> = ({
    data,
    error,
    setData,
    setError,
    setStage,
    stage,

    setLoading,
}) => {
    return (
        <div class='phone-stage' classList={{ code: stage() === 'code' }}>
            <div class='header'>
                <h2 class='title'>به دوزار خوش آمدید.</h2>
                <p class='title_small desc'>
                    لطفا شماره تلفن خود برای دریافت کد فعال‌سازی وارد کنید.
                </p>
            </div>
            <div class='phone-input'>
                <div class='holder title_small'>
                    <PhoneIcon />
                    شماره موبایل
                </div>
                <input
                    type={'number'}
                    inputmode={'numeric'}
                    maxlength='11'
                    class='title_small'
                    classList={{ inpError: error() !== '' }}
                    placeholder='مثال: 09123456789'
                    value={data.phone}
                    oninput={e => {
                        if (error()) setError('')
                        return setData({
                            phone: e.currentTarget.value,
                        })
                    }}
                />
                <div class='clear-input' onclick={() => setData({ phone: '' })}>
                    {data.phone.length >= 1 && <CloseFillIcon size={20} />}
                </div>
                <div class='error title_small'>
                    {error() !== '' && (
                        <>
                            <WarnnigIcon />
                            {error()}
                        </>
                    )}
                </div>
            </div>
            <button
                class='submit-phone basic-button title'
                classList={{ disable: data.phone.length !== 11 }}
                onclick={() => {
                    if (data.phone.length !== 11)
                        return setError('شماره تلفن خود را به درستی وارد کنید!')
                    if (data.phone[0] !== '0')
                        return setError(
                            'شماره تلفن خود را با پیش شماره 0 وارد کنید'
                        )

                    if (!phoneRegex.test(data.phone))
                        return setError('شماره تلفن خود را به درستی وارد کنید!')

                    return setStage('code')
                }}
            >
                ارسال کد
            </button>
        </div>
    )
}

const CODE_LENGTH = 5

const CodeStage: Component<LoginChildProps> = ({
    data,
    error,
    setData,
    setError,
    setStage,
    stage,

    loading,
    setLoading,
}) => {
    const [code, setCode] = createSignal([])

    const setBlock = () => {
        let codeInp = document.querySelector('.code-input') as HTMLElement
        let blocks = document.querySelectorAll('.block')
        let block = document.querySelector('.block') as HTMLElement
        let blockWidth = block.getBoundingClientRect().width

        let length = code().length

        console.log(blocks)

        if (length <= 0) {
            codeInp.style.left = `0px`
            codeInp.style.transform = `translateX(0)`
        } else {
            let left =
                blocks[
                    Math.min(CODE_LENGTH - 1, length)
                ].getBoundingClientRect().left

            codeInp.style.transform = `translateX(0)`

            if (innerWidth >= 768) {
                codeInp.style.left = `${left - blockWidth - innerWidth / 4}px`
            } else {
                codeInp.style.left = `${left - blockWidth - 10}px`
            }
        }
    }

    onMount(() => {
        const startUp = () => {
            let codeInp = document.querySelector('.code-input') as HTMLElement

            let block = document.querySelector('.block') as HTMLElement

            let blockWidth = block.getBoundingClientRect().width
            let blockHeight = block.getBoundingClientRect().height

            codeInp.style.width = `${blockWidth}px`
            codeInp.style.height = `${blockHeight}px`
        }

        startUp()

        window.onresize = () => {
            startUp()
            setBlock()
        }
    })

    createEffect(() => {
        setBlock()
    })

    return (
        <div
            class='code-stage'
            classList={{ code: stage() === 'code', loading: loading() }}
        >
            <button onclick={() => setStage('phone')} class='go-back'>
                <GoBackIcon size={28} />
            </button>
            <div class='header'>
                <h2 class='title'>کد فعال‌سازی را وارد کنید.</h2>
                <p class='title_small desc'>
                    کد 4 رقمی برای شماره {data.phone} ارسال شد.
                </p>
                <button
                    class='basic-button title_small change-number'
                    onclick={() => setStage('phone')}
                >
                    ویرایش شماره موبایل
                </button>
            </div>
            <div class='code-input-container'>
                <div class='input-wrapper'>
                    <input
                        type='text'
                        autocomplete='one-time-code'
                        inputmode='numeric'
                        class='section_title code-input'
                        oninput={e => {
                            const value = e.currentTarget.value.replace(' ', '')
                            const length = value.length

                            if (length > CODE_LENGTH) {
                                return (e.target.value = ' ')
                            }
                            if (e.inputType === 'deleteContentBackward') {
                                setCode(state => {
                                    state.pop()

                                    return state.slice()
                                })

                                return (e.target.value = ' ')
                            }

                            if (code().length > CODE_LENGTH)
                                return (e.target.value = ' ')

                            setCode([...code(), value || e.target.value])

                            e.target.value = ' '
                        }}
                        onpaste={e => {
                            let paste = e.clipboardData.getData('text')

                            let reg = /^\d+$/

                            if (!reg.test(paste)) return

                            paste.split('').map(data => {
                                setCode(s => {
                                    if (s.length >= CODE_LENGTH) return s
                                    return [...s, data]
                                })
                            })

                            e.currentTarget.value = ' '

                            return
                        }}
                    />
                    <div class='blocks'>
                        {Array.from(Array(CODE_LENGTH).keys()).map(
                            (_, index) => {
                                return (
                                    <div class='block section_title'>
                                        {code()[index] && (
                                            <div class='anim-code'>
                                                {code()[index]}
                                            </div>
                                        )}
                                    </div>
                                )
                            }
                        )}
                    </div>
                </div>
            </div>
            <button
                class='submit-code basic-button title'
                onclick={() => {
                    if (loading()) return

                    const btn = document.querySelector('.submit-code')
                    const btnClass = btn.className

                    if (btnClass.includes('anim')) return

                    btn.className += ' anim '

                    setTimeout(() => {
                        btnClass.replace('anim', '')

                        btn.className = btnClass
                    }, 1000)

                    setLoading(true)
                    return
                }}
            >
                ورود
            </button>
        </div>
    )
}

export default Login
