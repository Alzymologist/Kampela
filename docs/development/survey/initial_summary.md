# General information

(survey link)[https://cryptpad.fr/form/#/2/form/view/TUMvvTbGw2JpWbUanilxfIMgnW8UAfL4+KAvpyCk+UE/]

**Survey goal:** Define user groups, collect contacts for future surveys

# Hypothesis

From Parity Signer's development, it is know that:

1. All users are generally extremely secretive and unwilling to share their usage information, even privately and for common good
2. There are 3 general groups of users:
2.1. **"Stakeholder":** casual user with 1 key per device that only uses it for authorizations and multisignatures when managing large assets.
2.2. **"Power user":** has many accounts and complex derivation schemes, uses many features at once. Every setup is unique and significantly different from others.
2.3. **"Hacker":** wants to use cryptowallet with security features overriden, modify it, does not want safeduards.

# Data collection

The survey was distributed in Russian and English versions in social networks relevant to ecosystem development. Total 24 responses were collected over approximately 3 weeks time, which confirms H1. 4 explicit contacts for future analysis were acquuired.

# Data analysis

With this limited amount of data, detailed statistics is not feasible.

## General information

All respondents use cryptocurrency in one way or another.

9/24 respondents never used Substrate ecosystem, notably there are no developers in this group.

Other hardware storage tools mentioned were:

- Parity Signer
- Ledger/Ledger Nano
- Trezor
- AirGap Wallet

## Derivations usage

Out of all respondents, 8/24 claimed not to use derivations, 4/24 did not know what it is. Notably, no responses were collected claiming use of derivations outside of Substrate ecosystem, although it's not ecosystem-unique.

Out of users within Substrate ecosystem, 3/15 responded that they do not use derivations, and 3/15 refused to tell. One user in that group did not know what derivations are.

No significant correlation was observed between user role in ecosystem and use of derivations. This is certainly due to lack of data, but it is also quite clear that there are 1-address developer users and multiadress stakeholders.

## Suggestions

Users have noted poor usability of hardware wallets.

As always in similar discussion, hassle of metadata updates and lack of available manuals was noted. Demand for simpler version of everything was observed.

Ledger's low rate of updates was mentioned as unacceptable.

Users want support of Shamir secret sharing integrated.

Users care about whole system being opensource.

Users mentioned that they value mechanical robustness and quality of build.

One of the key demands for key managers noted from the feedback is a deep integration into existing wallets, as opposed to making a new app. Wallets development naturally lags behind the chains development, and adding more complexity on top of that would significantly decrease the usability of Kampela.

# Conclusions

1. We need to keep collecting information. Fortunately, the project is organized in a way that we could easily keep collecting data and reevaluating analysis till the end, relying on the latest information to plan the next stage.

2. H1 seems to be confirmed. There is still hope that users would become more open and willing to cooperate as project moves through milestones 3 and 4 delivering more tangible and visualizable results and we will suddenly have more high quality information.

3. 4 competing well-known key management systems were identified. We should learn from those systems, try to adapt their strong sides and fix their shortcomings.

4. We should limit the need to update firmware or perform any complicated metadata updates. Ideally Kampela should be usable at all times and in case of global breach just be recalled or replaced by users.

5. We should focus on standard communication protocol and integration into existing wallets (at least Nova who actively collaborates and helped us gather data for this report). We could also synergize this project into our proposal in making for a functionality-centered wallet project Kiondo.

6. We should remember to include banana split recovery protocol on-board.

7. Users do not seem to differentiate by roles in ecosystem, but rather by their technical skill and expected attack routes. We should reconsider user types classification. Fortunately, we have a list of user desires, and we can collect more data on this later, since we have a couple of milestones ahead before complete prototype design.

8. Users need to be forced to see the manual. Indeed, with hardware distribution, we could include instruction manual printouts with delivered package and make people want to read it. At the same time, large fraction of tutorials should be devoted to companion apps - wallets, we should come up with strategy on improving their clarity or at least somehow facilitate it.
