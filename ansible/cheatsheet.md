# Ansible Cheat Sheet

## Getting Help

List all available modules:

```sh
ansible-doc -l
```

Get help for module `foo`:

```sh
ansible-doc foo
```

## Examining Options

List all available options:

```sh
ansible-config list
```

Show description for `FOO` and `BAR` options:

```sh
ansible-config list FOO BAR
```

Show only options that changed:

```sh
ansible-config dump -v --only-changed
```

## Managing Inventory

List a members of `foo` from `inventory`:

```sh
ansible foo -i inventory --list-hosts
```

Show `<inventory>` in JSON:

```sh
ansible-inventory -i <inventory> --list
```

## Running Ad-Hoc Commands

Run `<module>` on `<hosts>` with arguments `<args>` as privileged `<user>`.
Use `<inventory>` as inventory:

```sh
ansible <hosts> -i <inventory> -m <module> -b -a <args> -u <user>
```

Investigate facts:

```sh
ansible <hostname> -i <inventory_file> -m setup
```

Run `<command>` on `<hosts>` as privileged user, connect to `<hosts>` as
`<user>`:

```sh
ansible <hosts> -m command -b -a '<command>' -u <user>
```

Shorter version of above:

```sh
ansible <hosts> -b -a '<command>' -u <user>
```

Check `foohost` is reachable:

```sh
ansible foohost -m ping
```

Check `foohost` is reachable as `root`:

```sh
ansible foohost -m ping --become
```

Show facts on `foohost` that match `ansible_mem*` pattern:

```sh
ansible foohost -m setup -a "filter=ansible_mem*"
```

### Useful commands

Protect `foo` against unprivileged access:

```sh
chmod 0600 foo
```

Get the name of the host:

```sh
/usr/bin/hostname
```

Provide information about logged user:

```sh
id
```

Show a time of last system boot:

```sh
who -b
```

Show security context of `foo`:

```sh
ls -Z foo
```

Show free disk space:

```sh
df
```

Show available system memory:

```sh
free -m
```

Show the directory structure of `foo`:

```sh
tree foo
```

Show the directory structure of the current working directory. Show only the
files or directories that matches `foo`. Highlight the file types:

```sh
tree -F -P foo
```

List block devices:

```sh
lsblk
```

Measure how long `foo` is executed:

```sh
time foo
```

Prefer password authentication to `foo.bar.baz` for the user `jdoe`:

```sh
ssh -o PreferredAuthentications=password jdoe@foo.bar.baz
```

Send 7 packets to `foo.bar.baz`:

```sh
ping -c 7 foo.bar.baz
```

Test if SMTP service `foo.bar.baz` is listening on port TCP/25 (to exit from
`telnet`, type `quit`):

```sh
telnet foo.bar.baz 25
```

Check if `foo.bar.baz` is available (HTTP):

```sh
curl foo.bar.baz
```

Check if `https://foo.bar.baz` (HTTPS) is available. Skip SSL certificate
verification, use third level of verbosity:

```sh
curl -k -vvv https://foo.bar.baz
```

Check if `https://foo.bar.baz` (HTTPS) is available for the user `jdoe`. Skip
SSL certificate verification:

```sh
curl -k https://foo.bar.baz -u jdoe
```

Check if `http://foo.bar.baz` (HTTP) is available. Show error details:

```sh
curl -S http://foo.bar.baz
```

Check the state of `foo` service:

```sh
systemctl status foo
```

Check if `foo` service is active:

```sh
systemctl is-active foo
```

Check if `foo` service is enabled:

```sh
systemctl is-enabled foo
```

Show the current default boot target:

```sh
systemctl get-default
```

List services enabled by firewall:

```sh
firewall-cmd --list-services
```

Display a timezone based on provided information given interactivelly:

```sh
tzselect
```

Show current time settings:

```sh
timedatectl status
```

Show known time zones:

```sh
timedatectl list-timezones
```

Show if python is installed and which version:

```sh
yum list installed python
```

## Managing Playbooks

Run `playbook.yml`:

```sh
ansible-playbook playbook.yml
```

Check the syntax of `playbook.yml`:

```sh
ansible-playbook playbook.yml --syntax-check
```

List all tasks in `playbook.yml` to be executed:

```sh
ansible-playbook playbook.yml --list-tasks
```

Start executing at task `Name of Task` in `playbook.yml`:

```sh
ansible-playbook playbook.yml --start-at-task 'Name of Task'
```

For every task, ask for confirmation to execute it:

```sh
ansible-playbook playbook.yml --step
```

Run `playbook.yml` in check mode:

```sh
ansible-playbook playbook.yml --check
```

Run `playbook.yml` in check mode, show differences between current and changed
managed files:

```sh
ansible-playbook playbook.yml --diff --check
```

## Managing Roles

Create a skeleton of `foo` role:

```sh
mkdir roles
cd roles
ansible-galaxy init foo
```

Install role from `./roles/requirements.yml`:

```sh
ansible-galaxy install -r roles/requirements.yml -p roles/
```

Use `--force` to overwrite previously installed `foo` role:

```sh
ansible-galaxy install -r roles/requirements.yml --force -p roles/
```

List local roles:

```sh
ansible-galaxy list
```

Remove local role `foo`:

```sh
ansible-galaxy remove foo
```

## Variables

```
ansible_facts                - Ansible facts (gathered by setup module)
ansible_facts.ansible_local  - custom facts
ansible_facts.date_time.date - current date
ansible_facts.devices        - host devices
ansible_facts.lvm            - host LVM settings
ansible_hostname             - short host name
ansible_fqdn                 - fully qualified host name
ansible_default_ipv4         - host IP address
ansible_distribution         - host distribution name
ansible_kernel               - host kernel version
ansible_machine              - host machine architecture
ansible_memtotal_mb          - total memory on host (MiB)
ansible_memfree_mb           - available memory on host (MiB)
```

## Modules

### Commands

Run command `foo`:

```yaml
command: foo
```

Create a GPT disk label on `/dev/vdb`, that starts 1MiB from the beginning of
the device and ends at the end of the device. Skip this step if `/dev/vdb1` has
already been created:

```yaml
command: >
  parted --script /dev/vdb mklabel gpt mkpart primary 1MiB 100%
args:
  created: /dev/vdb1
```

Install `python` to host machines:

```yaml
raw: dnf install python
```

Run script `foo` (script `foo` is copied to managed hosts from control node):

```yaml
script: foo
```

Run shell code:

```yaml
shell: "echo foo | grep foo > foo"
```

### Database

Ensure a user `jdoe` is in MySQL database and his password is `foo`:

```yaml
mysql_user:
  name: jdoe
  password: foo
```

### File

#### `blockinfile`

Ensure two lines `bar` and `baz` are present if `foo`:

```yaml
blockinfile:
  path: foo
  block: |
    bar
    baz
  state: present
```

The lines `bar` and `baz` are surrounded by commented block markers to ensure
idempotency. The `marker` parameter can change the markers:

```yaml
blockinfile:
  path: foo
  block: |
    bar
    baz
  state: present
  marker: "// {mark} OF FOO MANAGED BLOCK"
```

#### `copy`

Ensure `foo` exists and it is a copy of `bar`:

```yaml
copy:
  src: bar
  dest: foo
```

Ensure `foo` exists and its content is `Hello, World!\n`:

```yaml
copy:
  content: "Hello, World!\n"
  dest: foo
```

Ensure `foo` exists in `/etc/bar` directory with content `foo\n` and SELinux
type `etc_t`:

```yaml
copy:
  content: "foo\n"
  dest: /etc/bar/foo
  setype: etc_t
```

Ensure `foo` exitst with group and owner `root` and `644` file permissions and
it is a copy of `bar`:

```yaml
copy:
  src: bar
  dest: foo
  owner: root
  group: root
  mode: 0644
```

Ensure `foo` exists and it is a copy of `bar`. Copy `bar` only once:

```yaml
copy:
  src: bar
  dest: foo
  force: no
```

Ensure `foo` is a copy of `bar` and it is owned by user and group `jdoe`, has
file permissions `u+rw,g-wx,o-rwx` and SELinux context type `samba_share_t`:

```yaml
copy:
  src: bar
  dest: foo
  owner: jdoe
  group: jdoe
  mode: "u+rw,g-wx,o-rwx"
  setype: samba_share_t
```

#### `fetch`

Ensure `/etc/foo` from the host `foo.bar.baz` is copied to
`/home/jdoe/backups/foo.bar.baz/etc/foo`:

```yaml
fetch:
  src: /etc/foo
  dest: /home/jdoe/backups
  flat: no
```

#### `file`

Ensure `foo` exists and it is a directory:

```yaml
file:
  path: foo
  state: directory
```

Ensure a directory structure `foo/bar/baz` exists:

```yaml
file:
  path: foo/bar/baz
  recurse: yes
  state: directory
```

Ensure `/home/jdoe/foo` is a symbolic link to `/etc/bar`:

```yaml
file:
  src: /etc/bar
  dest: /home/jdoe/foo
  state: link
```

Ensure `/etc/bar` is a directory owned by `root` with SELinux type set to
`etc_t` and file permissions set to `755`:

```yaml
file:
  path: /etc/bar
  owner: root
  group: root
  mode: 0755
  state: directory
  setype: etc_t
```

Ensure a directory `foo` exists and it is owned by user and group `root` with
`500` file permissions:

```yaml
file:
  path: foo
  state: directory
  owner: root
  group: root
  mode: 0500
```

Ensure `foo` exists, it is owned by user `jdoe` and group `bar`, has
permissions `640` and its modification time is updated:

```yaml
file:
  path: foo
  owner: jdoe
  group: bar
  mode: 0640
  state: touch
```

Ensure `foo` has SELinux content type set to `samba_share_t` (behaves like
`chcon -t samba_share_type foo`). This change can be reverted back by
`restorecon foo`:

```yaml
file:
  path: foo
  setype: samba_share_t
```

Ensure `foo` is not present on managed host:

```yaml
file:
  path: foo
  state: absent
```

Ensure `foo`'s SELinux context has default values:

```yaml
file:
  path: foo
  seuser: _default
  serole: _default
  setype: _default
  selevel: _default
```

#### `lineinfile`

Ensure a file `foo` exists and contains a line `bar`:

```yaml
lineinfile:
  path: foo
  line: bar
  state: present
```

Allow `foo` group members to use `sudo` without password:

```yaml
lineinfile:
  path: /etc/sudoers
  regexp: "^%foo"
  line: "%foo ALL=(ALL) NOPASSWD: ALL"
  state: present
```

#### `stat`

Gather `foo`'s statistics and save them to `bar`:

```yaml
stat:
  path: foo
register: bar
```

Print `foo`'s MD5 checksum:

```yaml
- stat:
    path: foo
    checksum_algorithm: md5
  register: result

- debug:
    msg: "MD5('foo') == {{ result.stat.checksum }}"
```

#### `synchronize`

Synchronize `/path/to/foo` on managed hosts with `bar` on local machine
(similar to `rsync bar jdoe@foo.bar.baz:/path/to/foo`):

```yaml
synchronize:
  src: bar
  dest: /path/to/foo
```

#### `template`

Ensure `/etc/sudoers` is made from a `sudoers.j2` template. Before deploying it
to manage hosts validate its content by `/usr/bin/visudo -cf FILE`:

```yaml
template:
  src: sudoers.j2
  dest: /etc/sudoers
  validate: "/usr/bin/visudo -cf %s"
```

Ensure `foo` is made from a `foo.j2` and has owner and group set to `jdoe` and
file permissions set to `644`:

```yaml
template:
  src: foo.j2
  dest: foo
  owner: jdoe
  group: jdoe
  mode: 0644
```

Ensure `/etc/foo/foo` is made from a `foo.j2` and is owned by `root` with a
file permissions set to `600` and SELinux type to `etc_t`:

```yaml
template:
  src: foo.j2
  dest: /etc/foo/foo
  owner: root
  group: root
  mode: 0600
  setype: etc_t
```

### Net Tools

#### `get_url`

Ensure `foo.html` exists with `644` permissions and it was downloaded from
`http://foo.bar.baz/foo.html`:

```yaml
get_url:
  url: http://foo.bar.baz/foo.html
  dest: foo.html
  mode: 0644
```

Ensure `foo.cfg` exists and it is owned by user and group `bar`, it was
downloaded from `http://foo.bar.baz/foo.cfg` and its content is up-to-date:

```yaml
get_url:
  url: http://foo.bar.baz/foo.cfg
  dest: foo.cfg
  owner: bar
  group: bar
  force: yes
```

#### `uri`

Ensure `http://foo.bar.baz` is available and returns content with status code
`200`:

```yaml
uri:
  url: http://foo.bar.baz
  return_content: yes
  status_code: 200
```

Ensure `http://foo.bar.baz` is available and returns content:

```yaml
uri:
  url: http://foo.bar.baz
  return_content: yes
```

Ensure `http://foo.bar.baz` is available and returns status code `200`:

```yaml
uri:
  url: http://foo.bar.baz
  status_code: 200
```

### Packaging

#### `package`

Ensure `foo` is installed:

```yaml
package:
  name: foo
  state: present
```

#### `package_facts`

Gather facts about installed packages (can be accessed via
`ansible_facts.packages`). Choose package manager automatically depending on
the host system:

```yaml
package_facts:
  manager: auto
```

#### `rpm_key`

Ensure repository public key from `https://foo.bar.baz/RPM-GPG-KEY` is present
on the host:

```yaml
rpm_key:
  key: https://foo.bar.baz/RPM-GPG-KEY
  state: present
```

#### `yum`

Ensure `foo` is installed:

```yaml
yum:
  name: foo
  state: present
```

Ensure the latest `bar` and `baz` are installed:

```yaml
yum:
  name:
    - bar
    - baz
  state: latest
```

Ensure `foo` is not installed:

```yaml
yum:
  name: foo
  state: absent
```

#### `yum_repository`

Ensure `yum` repository with name `foo`, description `Foo Inc.`, base url
`https://foo.bar.baz/os`, and with GPG checking enabled is stored in
`/etc/yum.repos.d/foo.repo`:

```yaml
yum_repository:
  name: foo
  description: Foo Inc.
  baseurl: https://foo.bar.baz/os
  gpgcheck: yes
  file: foo
```

### System

#### `at`

Ensure there is a task scheduled that runs every minute `touch ~/foo`. Do not
add new command to the `at` queue if there is already matching one:

```yaml
at:
  command: "touch ~/foo"
  count: 1
  units: minutes
  unique: true
  state: present
```

#### `authorized_key`

Ensure `jdoe` has his SSH public key `jdoe.key.pub` on remote host:

```yaml
authorized_key:
  user: jdoe
  key: jdoe.key.pub
```

#### `cron`

Ensure there is a `/etc/cron.d/add-stamp` crontab file that schedules a job
that runs as the `jdoe` user and writes current date and time every 2 minutes
between 4:00 and 14:59 on Tuesday through Thursday to `stamp` file in `jdoe`'s
home directory:

```yaml
cron:
  name: Add time and date to /home/jdoe/stamp
  user: jdoe
  cron_file: add-stamp
  minute: "*/2"
  hour: 4-15
  day: 2-4
  job: date >> /home/jdoe/stamp
  state: present
```

Ensure there is no `Add time and date to /home/jdoe/stamp` cron job in
`/etc/cron.d/add-stamp` crontab file:

```yaml
cron:
  name: Add time and date to /home/jdoe/stamp
  cron_file: add-stamp
  state: absent
```

#### `filesystem`

Ensure logical volume `bar-lv` which is part of `foo-vg` volume group is
formatted as `ext4`:

```yaml
filesystem:
  dev: /dev/foo-vg/bar-lv
  fstype: ext4
```

Ensure `ext4` file system exists on `/dev/sda1`. Do not force its creation:

```yaml
filesystem:
  dev: /dev/sda1
  fstype: ext4
  force: no
```

#### `firewalld`

Ensure HTTP service is permanently enabled and immediately ready to use:

```yaml
firewalld:
  service: http
  permanent: true
  state: enabled
  immediate: yes
```

#### `group`

Ensure the group `foo` exists:

```yaml
group:
  name: foo
  state: present
```

#### `lvg`

Ensure `foo-vg` is a volume group that contains `/dev/vda1`:

```yaml
lvg:
  vg: foo-vg
  pvs: /dev/vda1
```

#### `lvol`

Ensure logical volume `bar-lv` is a part of `foo-vg` volume group and has size
64MiB:

```yaml
lvol:
  lv: bar-lv
  vg: foo-vg
  size: 64MiB
```

Ensure logical volume `bar-lv` which is part of `foo-vg` volume group has the
storage capacity of 64MiB. Force the expansion:

```yaml
lvol:
  lv: bar-lv
  vg: foo-vg
  size: 64MiB
  resizefs: yes
  force: yes
```

#### `mount`

Ensure logical volume `bar-lv` formatted as `ext4` which is part of `foo-vg`
volume group is mounted at `/var/bar` mount point persistently:

```yaml
mount:
  path: /var/bar
  src: /dev/foo-vg/bar-lv
  fstype: ext4
  state: mounted
```

Ensure `/etc/fstab` mounts `/dev/sda1`, formatted as `ext4`, on `/mnt/foo` at
boot and it is currently mounted. Also allow backups and filesystem checks:

```yaml
mount:
  path: /mnt/foo
  src: /dev/sda1
  fstype: ext4
  state: mounted
  # 0 - do not backup with dump, 1 - otherwise
  dump: 1
  # Order of filesystem check performed by fsck
  # (0 - don't check, 1 - boot drives, 2 - other drives)
  passno: 2
```

#### `parted`

Ensure `/dev/vda` has a partition number 1 of a size of 256MiB that starts at
1MiB offset:

```yaml
parted:
  device: /dev/vda
  number: 1
  part_start: 1MiB
  part_end: 257MiB
  state: present
```

#### `reboot`

Reboot a machine:

```yaml
reboot:
```

#### `sefcontext`

Ensure SELinux type of `foo` is changed persistently to `samba_share_t` (this
updates only SELinux policy, like `semanage fcontext -a -t samba_share_t foo`,
but not `foo` file itself):

```yaml
sefcontext:
  target: foo
  setype: samba_share_t
  state: present
```

#### `service`

Ensure the service `foo` is running:

```yaml
service:
  name: foo
  state: started
```

Ensure the service `foo` is running and enabled:

```yaml
service:
  name: foo
  state: started
  enabled: true
```

Ensure the service `foo` is enabled and restarted:

```yaml
service:
  name: foo
  enabled: true
  state: restarted
```

Ensure the service `foo` is disabled and stopped:

```yaml
service:
  name: foo
  enabled: no
  state: stopped
```

#### `setup`

Gather facts manually:

```yaml
setup:
```

#### `timezone`

Ensure the time zone is UTC:

```yaml
timezone:
  name: UTC
```

#### `user`

Ensure user `jdoe` exists with UID 4000:

```yaml
user:
  name: jdoe
  uid: 4000
  state: present
```

Ensure user `jdoe` exists and is a member of `foo` group:

```yaml
user:
  name: jdoe
  groups: foo
  state: present
```

Ensure user `jdoe` exists and is a member of `foo` group. User `jdoe` can be
also a member of any other groups:

```yaml
user:
  name: jdoe
  groups: foo
  state: present
  append: yes
```

Ensure user `jdoe` exists and his account is protected by password `bar` stored
as SHA-512 hash with a salt `baz`:

```yaml
user:
  name: jdoe
  state: present
  password: "{{ 'bar' | password_hash('sha512', 'baz') }}"
```

Ensure user `jdoe` does not exist:

```yaml
user:
  name: jdoe
  state: absent
  force: yes
  remove: yes
```

### Utilities

#### `assert`

Fail if `(foo && !bar)` is false. Print `Assertion (foo && !bar) failed!` on
fail or `Assertion (foo && !bar) succeeded.` otherwise:

```yaml
assert:
  that:
    - foo
    - not bar
  fail_msg: Assertion (foo && !bar) failed!
  success_msg: Assertion (foo && !bar) succeeded.
```

#### `debug`

Print the value of `foo` variable:

```yaml
debug:
  var: foo
```

Print the value of `foo` only if verbosity level is greater or equal to 2
(`ansible-playbook -vv`):

```yaml
debug:
  var: foo
  verbosity: 2
```

Print a `bar`:

```yaml
debug:
  msg: bar
```

#### `fail`

Fail with `foo` message:

```yaml
fail:
  msg: foo
```

#### `set_fact`

Ensure fact `foo` with a value `bar` is present:

```yaml
set_fact:
  foo: bar
```

### Web Infrastructure

#### `htpasswd`

Ensure a file `/etc/httpd/secrets/htpasswd` is containing user `foo` with
password `bar`. The file `/etc/httpd/secrets/htpasswd` is owned by user and
group `apache` and has `400` permissions:

```yaml
htpasswd:
  path: /etc/httpd/secrets/htpasswd
  name: foo
  password: bar
  owner: apache
  group: apache
  mode: 0400
```
