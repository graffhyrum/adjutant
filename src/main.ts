import {invoke} from "@tauri-apps/api/core";
import Quill from 'quill';
import "quill/dist/quill.core.css";


window.onload = () => {
  function buildWelcomeMessage() {
    const greetingEl = document.getElementById('greeting');
    if (!greetingEl) return;
    
    const currentHour = new Date().getHours();
    let greetingText = 'Hello';
    
    if (currentHour >= 5 && currentHour < 12) {
      greetingText = 'Good Morning';
    } else if (currentHour >= 12 && currentHour < 17) {
      greetingText = 'Good Afternoon';
    } else if (currentHour >= 17 || currentHour < 2) {
      greetingText = 'Good Evening';
    }
    
    greetingText += ', what would you like to share?';
    
    greetingEl.textContent = greetingText;
  }
  
  function attachTime() {
    const timeEl = document.getElementById('time');
    if (!timeEl) return;
    
    const updateTime = () => {
      const currentTime = new Date().toLocaleTimeString([], {hour: '2-digit', minute: '2-digit'});
      timeEl.textContent = currentTime;
    }
    
    updateTime();
    setInterval(updateTime, 1_000);
  }
  
  async function buildEnabledSocials() {
    function assertIsEnabledSocial(maybeSocial: unknown): asserts maybeSocial is Record<string, boolean> {
      if (typeof maybeSocial !== 'object' || maybeSocial === null) {
        throw new Error('Invalid socials object');
      }
    }
    
    
    const socials = await invoke('get_enabled_socials');
    assertIsEnabledSocial(socials);
    const platformsDiv = document.getElementById('platforms');
    
    // clear the platforms div
    while (platformsDiv?.firstChild) {
      platformsDiv.removeChild(platformsDiv.firstChild);
    }
    
    // create a label and an input for each social platform
    for (const [platform, enabled] of Object.entries(socials)) {
      console.log(platform, enabled);
      const label = document.createElement('label');
      label.htmlFor = platform;
      
      const checkbox = document.createElement('input');
      checkbox.type = 'checkbox';
      checkbox.id = platform;
      checkbox.name = 'platform';
      checkbox.value = platform;
      checkbox.checked = enabled;
      
      label.appendChild(checkbox);
      label.appendChild(document.createTextNode(platform));
      
      platformsDiv?.appendChild(label);
    }
  }
  
  buildWelcomeMessage();
  attachTime();
  buildEnabledSocials();
};

const quill = new Quill('#editor', {});

