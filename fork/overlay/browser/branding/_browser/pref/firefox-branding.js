// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// _Browser branding + Phase 1 default-pref overrides.
// These are DEFAULT prefs compiled into the build; users can still change them
// at runtime unless explicitly locked.

// ---- Branding / startup ----
pref("startup.homepage_override_url", "");
pref("startup.homepage_welcome_url", "");
pref("startup.homepage_welcome_url.additional", "");

// ---- Phase 1: strip unwanted built-ins ----

// Pocket integration off.
pref("extensions.pocket.enabled", false);

// Telemetry and data reporting off by default.
pref("toolkit.telemetry.enabled", false);
pref("toolkit.telemetry.unified", false);
pref("toolkit.telemetry.archive.enabled", false);
pref("datareporting.healthreport.uploadEnabled", false);
pref("datareporting.policy.dataSubmissionEnabled", false);
pref("app.shield.optoutstudies.enabled", false);
pref("browser.newtabpage.activity-stream.feeds.telemetry", false);
pref("browser.newtabpage.activity-stream.telemetry", false);

// Firefox Accounts / Sync not enabled as a default entry point.
// (Set to true to restore the built-in Sync UI.)
pref("identity.fxaccounts.enabled", false);
