// Show menu
const showMenu = (toggleId, navId) => { 
    const toggle = document.getElementById(toggleId),
    nav = document.getElementById(navId)

    if(toggle && nav) { 
        toggle.addEventListener('click', () => { 
            nav.classList.toggle('show-menu')
        })
    }
}
showMenu('nav-toggle', 'nav-menu')
//  Remove Menu Mobile 
const navLink = document.querySelectorAll('.nav__link')
function linkAction() { 
    const navMenu = document.getElementById('nav-menu')
    navMenu.classList.remove('show-menu')
}
navLink.forEach(n => n.addEventListener('click', linkAction))
// Scroll Sections Active lInk
const sections = document.querySelectorAll('section[id]')

function scrollActive() { 
    const scrollY = window.pageYOffset

    sections.forEach(current => { 
        const sectionHeight = current.offsetHeight
        const sectionTop = current.offsetTop - 50 
        sectionId = current.getAttribute('id')

        if(scrollY > sectionTop && scrollY <= sectionTop + sectionHeight) { 
            document.querySelectory('.nav__menu a[href*=' + sectionId + ']').classList.add('active-link')
        } else { 
            document.querySelectory('.nav__menu a[href*=' + sectionId + ']').classList.remove('active-link')

        }
    })
}
window.addEventListener('scroll', scrollActive)

// Change Background Header
function scrollHeader() { 
    const header = document.getElementById('header')
    if(this.scrollY >= 200) header.classList.add('scroll-header'); else header.classList.remove('scroll-header')
}

window.addEventListener('scroll', scrollHeader)

// Show Scroll TOp
function scrollTop() { 
    const scrollTop = document.getElementById('scroll-top')
    if(this.scrollY >= 560) scrollTop.classList.add('show=scroll'); else scrollTop.classList.remove('show-scroll')
}

window.addEventListener('scroll', scrollTop)

