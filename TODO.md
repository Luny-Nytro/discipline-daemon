# Use script injection
Filtre web content that cannot be filtered statically (before it reaches the client) in the client itself by injecting a javascript tag in the html file that depends on that web content.

Make it hard for users to figure out that some filtering is done on the client and using script injection in order to make it harder for them to know about the inner workings of discipline and thus being able to disable it.

Ideas:
  - injected scripts must be minified, stripped from comments and identifiers must be shortened or randomized to make it harder for outsiders to what the script does.
  - everytime a script is injected, it must be uglified in a a different way so that outsiders cannot detect and remove our injected scripts by statically analysing the containing html file.
  - remove the script tag containing the web filtering javascript code injected by discipline.

# Install to a random location
Everytime the discipline service is installed, it should be installed in a different location with the executable and database file names being also random in order to make it harder to find and disable.

# Use web browser extensions.
Consider doing more advanced and more dynamic filtering by utilising web browser extensions. Create a web browser extension that allows for more advanced and more dynamic filtering and somehow install the extension when the discipline service is installed. Also, make the extension uninstallable.

# Don't trust system and network time
Create a clock that keeps track of time.
Create the ablity to sync time from within discipline from an ntp server.
If the system time is suddenly way before the last recorded time, start up the clock with the last record time

# Denial-of-Service Attack
Discipline must never crush under any circumstances so all modules in the '/db' directory must protect against Denial-of-Service Attacks by doing the following:
  - Limiting the number of items a store may store.
  - Limiting the depth of all Objs so that no Obj becomes too big.

# Prevent websites from download content
Create a rule feature to that prevents websites to download files on the client.

# Feature: Block css image uris
some image properties in css accept a uri that refers to an image. Such a property causes an image to be loaded. Remove those properties from the style sheet before it reaches the browser.

# Feature: Strip away embedded SVG images from html documents
some SVG images are just icons, which are not harmful. Figure out a way to avoid removing these
by accident.

One to do that is by checking the number of elements or depth of the SVG image. Icons typically are very small, this way, we can correctly detect most icons.

# Safty
The Discipline Service must NEVER panic or crush. Each and every error must be handled gracfully. The Discipline Service must be protected from common cyber-attacks like the Denial-of-Service Attack.

# Denial-of-Service Attack
Discipline must never crush under any circumstances so all modules in the '/db' directory must protect against Denial-of-Service Attacks by doing the following:
  - Limiting the number of items a store may store.
  - Limiting the depth of all Objs so that no Obj becomes too big.
  
# Feature: Web Delay
allow the user to specify a delay before the client gets the resource it requests.

# Necessary Os-level hacks
you need to disable the os's recovery env.
you need to disable users from changing the time manually.

# Feature: Log network events
A log of every interception is stored on the disk and the user can open a panel in the app
and see information about current and recent interceptions. 

The user should particulary see:
  - the http request's url
  - the http request initiation time
  - the http request headers
  - a loader indicating hoe much of the http request body is sent
  - the http response status
  - the http response headers
  - a loader indicating how much of the the http response body is recieved
  - the http response body
  - Information about any websocket connections or other protocol connections initiated through http CONNECT or http Upgrade should also sub-protocols should also be displayed.
  - whether the message is allowed
  - whether the message is blocked, and, if so, a reference to the object that blocked it should be provided


Create a youtube watch time feature: It restricts how much time you spend watching youtube videos.

Create a twitter read-limit feature: It restricts how many tweets you can read.

Create a reddit post-limit feature: It restricts how many posts you can view.

Create an instagram post-limit feature: It restricts how many posts you view.

Create a facebook post-limit feature: It restricts how many posts you view.

Create a feature that can block/allow specific search suggestions in:
  - Google Search
  - Bing Search
  - YouTube Search
  - And other pltforms

# Feature: Puase delay opening particular websites and apps
when specified websites or apps are opened, don't let them open directly, but delay that for a specific duration.