const PureOSLaunchManifest = {
  pureos_release: 'v17.46Y',
  stage: 'debug_apk_handoff_prism_launch',
  screen: 'minimal_prism_launch',
  action: 'launch_pureos',
  required_visible_text: ['PURE OS', 'Launch Pure OS'],
  next_route: 'pureos_runtime_home',
  truth_boundary: 'v17.46Y provides a build-ready Android project and install handoff. The APK appears only after Gradle/Android Studio runs on a machine with Android SDK access.'
};

const PureOSLaunch = {
  boot() {
    const launch = document.getElementById('launch-button');
    launch.addEventListener('click', () => this.launch());
    launch.addEventListener('keydown', (event) => {
      if (event.key === 'Enter' || event.key === ' ') this.launch();
    });
  },
  launch() {
    const receipt = {
      version: PureOSLaunchManifest.pureos_release,
      event: PureOSLaunchManifest.action,
      next_route: PureOSLaunchManifest.next_route,
      at: new Date().toISOString()
    };
    localStorage.setItem('pureos:launch_receipt', JSON.stringify(receipt));
    document.body.classList.add('launching');
    setTimeout(() => { window.location.hash = '#pureos_runtime_home'; }, 360);
  }
};

window.PureOSLaunchManifest = PureOSLaunchManifest;
window.addEventListener('DOMContentLoaded', () => PureOSLaunch.boot());
