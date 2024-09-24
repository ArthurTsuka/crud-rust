import React from 'react';
import { ShimmerButtonDemo } from "./shimmerButton"
// import { AnimatedSubscribeButtonDemo } from "./subscribeButton"
import SignInButton from './signInButton'

const Navbar: React.FC = () => {
    return (
        <nav className="bg-customDark p-4">
            <div className="container mx-auto flex justify-between items-center">
                <div className=" flex text-white text-2xl font-bold items-center">
                    <h3>WTG</h3>
                    <img
                        src = '/axe.png'
                        alt='Logo Brand'
                        className='h-8 w-auto'
                    />
                </div>
                <ul className="flex space-x-6 items-center">
                    <li>
                        <a href="#home" className="text-white hover:text-gray-300">
                            Learn
                        </a>
                    </li>
                    <li>
                        <a href="#about" className="text-white hover:text-gray-300">
                            What's new
                        </a>
                    </li>
                    <li>
                        <a href="#services" className="text-white hover:text-gray-300">
                            Pricing
                        </a>
                    </li>
                    <li>
                        <a href="#contact" className="text-white hover:text-gray-300">
                            Blog
                        </a>
                    </li>
                    <li>
                        <SignInButton onClick={function (): void {
                        } } buttonText={'Sign in'} />
                    </li>
                    <li>
                        <ShimmerButtonDemo />
                    </li>
                </ul>
            </div>
        </nav>
    );
};

export default Navbar;
