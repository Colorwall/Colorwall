import React from 'react';
import EnhancedTitleBar from './components/titlebar';
import ModernNavigation from './components/Newnavigation';
import LoadingScreen from './components/LoadingScreen';
import HomePage from './pages/HomePage';
import LibraryPage from './pages/LibraryPage';
import StorePage from './pages/StorePage';
import SettingsPage from './pages/SettingsPage';
import ProfilePage from './pages/ProfilePage';
import UpdatesPage from './pages/UpdatesPage';
import FeedbackPage from './pages/FeedbackPage';
import { DLProvider } from './context/DownloadContext';
import { ConfirmProvider } from './context/ConfirmContext';
import { VisibilityProvider, useVisibility } from './context/WinCloseContext';
import FloatingProgress from './components/floatyfloaty';
import ContextMenu from './components/ContextMenu';
import { invoke } from '@tauri-apps/api/core';

function App() {
    const { isVisible } = useVisibility();
    const [isLoading, setIsLoading] = React.useState(true);
    const [activeTab, setActiveTab] = React.useState('home');
    // debounce guard — prevents rapid tab switches from stacking multiple re-renders
    const tabSwitchTimer = React.useRef<ReturnType<typeof setTimeout> | null>(null);
    const [browsingSource, setBrowsingSource] = React.useState<string | null>(null);
    const [browsingLive, setBrowsingLive] = React.useState(false);
    const [isDirectNavigation, setIsDirectNavigation] = React.useState(false);

    const [isSidebarExpanded, setIsSidebarExpandedRaw] = React.useState(() => {
        try {
            return localStorage.getItem('sidebar-expanded') === 'true';
        } catch {
            return false;
        }
    });

    // persist sidebar state to localstorage on every change
    const setIsSidebarExpanded = React.useCallback((expanded: boolean) => {
        setIsSidebarExpandedRaw(expanded);
        try {
            localStorage.setItem('sidebar-expanded', String(expanded));
        } catch { /* noop */ }
    }, []);
    const [updateAvailable, setUpdateAvailable] = React.useState(false);

    React.useEffect(() => {
        // Load global settings immediately to apply visual effects (vibrancy, theme)
        invoke('get_settings').then((res: any) => {
            if (res.success && res.settings) {
                if (res.settings.windowVibrancy) {
                    document.body.classList.add('vibrancy-enabled');
                } else {
                    document.body.classList.remove('vibrancy-enabled');
                }
                // apply performance mode classes before first paint
                if (res.settings.perfMode) {
                    if (!res.settings.perfBlurEnabled) document.body.classList.add('perf-no-blur');
                    if (!res.settings.perfAnimationsEnabled) document.body.classList.add('perf-no-animations');
                    if (!res.settings.perfShadowsEnabled) document.body.classList.add('perf-no-shadows');
                }
            }
        }).catch(err => {
            console.error("Failed to load global settings:", err);
        });

        const timer = setTimeout(() => {
            setIsLoading(false);
        }, 2500);

        return () => clearTimeout(timer);
    }, []);

    // background update check on app load
    React.useEffect(() => {
        const checkUpdate = async () => {
            try {
                const { getVersion } = await import('@tauri-apps/api/app');
                const currentVersion = await getVersion();
                const res = await fetch('https://www.colorwall.xyz/api/updates/check');
                if (res.ok) {
                    const data = await res.json();
                    const remote = (data.version || '').split('.').map(Number);
                    const local = currentVersion.split('.').map(Number);
                    for (let i = 0; i < Math.max(remote.length, local.length); i++) {
                        if ((remote[i] || 0) > (local[i] || 0)) { setUpdateAvailable(true); break; }
                        if ((remote[i] || 0) < (local[i] || 0)) break;
                    }
                }
            } catch (e) {
                console.error('background update check failed:', e);
            }
        };
        checkUpdate();
    }, []);

    const handleSourceNavigation = (source: string) => {
        setBrowsingSource(source);
        setBrowsingLive(false);
        setIsDirectNavigation(false);
        setActiveTab('browse');
    };

    const handleLiveNavigation = () => {
        setBrowsingLive(true);
        setBrowsingSource(null);
        setActiveTab('browse');
    };

    const handleSettingsClick = () => {
        setActiveTab('settings');
        setBrowsingSource(null);
        setBrowsingLive(false);
    };

    const handleUserClick = () => {
        setActiveTab('profile');
        setBrowsingSource(null);
        setBrowsingLive(false);
    };

    const handleTabChange = (tab: string) => {
        if (tabSwitchTimer.current) {
            clearTimeout(tabSwitchTimer.current);
            tabSwitchTimer.current = null;
        }

        tabSwitchTimer.current = setTimeout(() => {
            tabSwitchTimer.current = null;
            setActiveTab(tab);
            if (tab === 'browse') {
                setIsDirectNavigation(true);
            } else {
                setBrowsingSource(null);
                setBrowsingLive(false);
                setIsDirectNavigation(false);
            }
        }, 50);
    };

    // Deep unmount the entire UI when hidden to ensure zero GPU usage.
    if (!isVisible) {
        return null;
    }

    if (isLoading) {
        return <LoadingScreen />;
    }

    return (
        <div className="app-container" style={{ minHeight: '100vh', position: 'relative' }}>
            <ContextMenu />
            <EnhancedTitleBar onSettingsClick={handleSettingsClick} onUserClick={handleUserClick} />
            <ModernNavigation
                activeTab={activeTab}
                onTabChange={handleTabChange}
                isExpanded={isSidebarExpanded}
                onExpandChange={setIsSidebarExpanded}
                updateAvailable={updateAvailable}
            />

            <div style={{
                marginLeft: isSidebarExpanded ? '212px' : '72px',
                marginTop: '48px',
                minHeight: 'calc(100vh - 48px)',
                transition: 'margin-left 0.3s cubic-bezier(0.4, 0, 0.2, 1)'
            }}>
                {activeTab === 'home' && (
                    <HomePage
                        onNavigateToSource={handleSourceNavigation}
                        onNavigateToLive={handleLiveNavigation}
                        isActive={activeTab === 'home'}
                    />
                )}
                {activeTab === 'browse' && (
                    <StorePage
                        selectedSource={browsingSource || 'all'}
                        filterType={browsingLive ? 'live' : 'static'}
                        isDirectNavigation={isDirectNavigation}
                        onGoToLibrary={() => setActiveTab('library')}
                    />
                )}
                {activeTab === 'library' && (
                    <LibraryPage />
                )}
                {activeTab === 'settings' && (
                    <SettingsPage />
                )}
                {activeTab === 'profile' && (
                    <ProfilePage />
                )}
                {activeTab === 'updates' && (
                    <UpdatesPage />
                )}
                {activeTab === 'feedback' && (
                    <FeedbackPage />
                )}
            </div>
            <FloatingProgress />
        </div>
    );
}

const AppWrapper = () => (
    <VisibilityProvider>
        <ConfirmProvider>
            <DLProvider>
                <App />
            </DLProvider>
        </ConfirmProvider>
    </VisibilityProvider>
);

export { AppWrapper as default };
