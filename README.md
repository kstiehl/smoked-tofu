# 🔥 Smoked Tofu 🥢

*Because we're smoking that infrastructure code like it's tofu night at the vegan BBQ* 

## What's This All About? 🤔

Smoked Tofu is a GitHub webhook server that's here to automate your OpenTofu (formerly Terraform) workflows! 🚀 

The name comes from the beautiful evolution: Terraform → OpenTofu → Smoked Tofu. It's like watching your infrastructure tooling grow up and get a hipster makeover! ✨

## Features 🎯

- 🎣 **GitHub Webhook Handler**: Catches those sweet, sweet push events
- 🏃‍♂️ **Command Execution**: Runs your OpenTofu/Terraform commands automatically 
- ✅ **GitHub Check Runs**: Updates your PRs with success/failure status
- 🔒 **Secure**: HMAC-SHA256 signature verification protects against unauthorized requests
- 🦀 **Rust Powered**: Because we like our infrastructure tools fast and reliable
- 🔧 **Configurable**: Bring your own commands, we'll smoke 'em for you

## How It Works 🛠️

1. Someone pushes code to your repo 📝
2. GitHub sends a webhook to Smoked Tofu 📡
3. Smoked Tofu executes your configured command (probably `tofu plan` or `tofu apply`) 🏗️
4. Results get posted back to GitHub as check runs ✅❌
5. Your infrastructure gets updated, and everyone's happy! 🎉

## Getting Started 🚀

```bash
# Build the tofu smoker
cargo build --release

# Fire it up!
./target/release/smoked-tofu \
  --token YOUR_GITHUB_TOKEN \
  --secret YOUR_WEBHOOK_SECRET \
  --command "tofu" plan
```

## Why "Smoked" Tofu? 🤷‍♀️

Well, when HashiCorp changed their license and the community forked Terraform into OpenTofu, we thought: "You know what? Let's take this tofu and smoke it real good!" 

Plus, smoked tofu is delicious, and so is automated infrastructure! 😋

## Configuration 🎛️

### GitHub Webhook Setup

1. Go to your repository → Settings → Webhooks → Add webhook
2. Set **Payload URL** to `http://your-server:port/webhook`
3. Set **Content type** to `application/json`
4. **Set a Secret** - this must match your `--secret` parameter for security!
5. Select events (usually just "Push events")

⚠️ **Security Note**: The webhook secret is required to prevent unauthorized requests from executing your commands.

---

*Made with ❤️ and probably too much coffee ☕*