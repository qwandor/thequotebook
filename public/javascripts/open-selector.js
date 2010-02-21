/*************************************************************
 * Open-selector
 *  Help regular people to login using OpenID without them knowing
 *
 * Javascript snippet to change your regular OpenID box for a provider 
 * combo box.
 *
 * Based on:
 *    - http://sites.google.com/site/oauthgoog/UXFedLogin
 *    - http://ma.gnolia.com/signin/ 
 *
 * This is an alternative to IdSelector? (http://www.idselector.com/)
 *  
 * Proyect page:
 *  http://code.google.com/p/open-selector/
 * 
 * Jj (jjdelc@gmail.com)
 *  http://isgeek.net/ - 2008
 *
 *
 * Usage
 * -----
 * Add this to your signin page:
 *
 * <script src="/js/open-selector.js" type="text/javascript"></script>
 * <script type="text/javascript">
 *     open_selector.openid_form_id= 'openid_form';// ID for the OpenID form
 *     open_selector.openid_box_id= 'openid_url';// ID for the OpenID URL box
 *     open_selector.init();
 * </script>
 *
 *************************************************************/

// List of OpenID providers
var providers = {
    livejournal: {
        name: 'LiveJournal',
        label: 'LiveJournal username',
        ask_username: true,
        icon: 'http://livejournal.com/favicon.ico',
        url: 'http://{username}.livejournal.com/'
    },

    yahoo: {
        name: 'Yahoo',
        info: 'Continue to Yahoo to login.',
        ask_username: false,
        icon: 'http://yahoo.com/favicon.ico',
        url: 'http://yahoo.com/'
    },

    google: {
        name: 'GMail account',
        ask_username: false,
        info: 'Sign in with Google.',
        icon: 'http://mail.google.com/mail/images/favicon.ico',
        url: 'https://www.google.com/accounts/o8/id'
    },

    aol: {
        name: 'AOL',
        label: 'AOL screenname',
        ask_username: true,
        icon: 'http://aol.com/favicon.ico',
        url: 'http://openid.aol.com/{username}/'
    },

    flickr: {
        name: 'Flickr',
        label: 'Flickr username',
        ask_username: true,
        icon: 'http://flickr.com/favicon.ico',
        url: 'http://flickr.com/{username}/'
    },

    technorati: {
        name: 'Technorati',
        label: 'Technorati username',
        ask_username: true,
        icon: 'http://technorati.com/favicon.ico',
        url: 'http://technorati.com/people/technorati/{username}/'
    },

    wordpress: {
        name: 'Wordpress',
        label: 'Wordpress.com username',
        ask_username: true,
        icon: 'http://wordpress.com/favicon.ico',
        url: 'http://{username}.wordpress.com/'
    },
    
    blogger: {
        name: 'Blogger',
        label: 'Blogger account',
        ask_username: true,
        icon: 'http://blogger.com/favicon.ico',
        url: 'http://{username}.blogspot.com/'
    },
    
    myopenid: {
        name: 'MyOpenID',
        ask_username: true,
        icon: "https://www.myopenid.com/favicon.ico?version=1",
        url: 'http://{username}.myopenid.com/'
    },

    openid: {
        name: 'Other OpenID provider',
        label: 'OpenID identifier',
        info: 'You know what OpenID is.',
        ask_username: true,
        icon: 'http://openid.net/favicon.ico',
        url: '{username}'
    }
};

var open_selector = {

    // OPTIONS

    // ID of the OpenID Login form
    openid_form_id: 'openid_form',

    // ID of the block containing the OpenID URL box and related content, which we will replace with our own
    // If this is false, it will default to openid_box_id.
    openid_block_id: false,

    // ID of the OpenID URL box
    openid_box_id: 'openid_url',

    // Set to false to hide Open-selector credits :(
    show_credits: true,

    // Set to false to hide the "Select provider" label before the combo
    show_label: true,

    // Username textbox input size
    textbox_size: 20,

    // Display the provider and user boxes inline
    inline: false,

    // Set to false to disable the updating of the OpenID URL while typing
    // improves performance
    update_openid_url: true,

    init: function(){
        if (this.openid_block_id == false) {
            this.openid_block_id = this.openid_box_id;
        }
        var openid_form = $('#' + this.openid_form_id);
        var openid_block = $('#' + this.openid_block_id);
        var openid_box = $('#' + this.openid_box_id);
        var openid_box_label = $('label[for=' + this.openid_box_id + ']');
        var button = $('#' + this.openid_form_id + ' input[type=submit]');

        var open_user_html = '<input type="text" style="padding-left: 22px;" id="open-selector-username" disabled="disabled" size="' + this.textbox_size + '"/>';
        var select_html = '<select id="open-selector"><option value="0">Select provider</option></select> ';
        var html_credits = this.show_credits?'<p style="color:#DDD;font-size:80%;font-style:italic;">Powered by <a href="http://open-selector.com" title="Open-selector">Open-selector</a>.</p>':'';
        var html_label = this.show_label?'<label for="open-selector">Account type</label>':'';

        openid_block.hide();
        openid_box_label.hide();

        if (this.inline) {
            button.before(select_html + open_user_html + '<span style="font-size:90%;font-style:italic;color:#BBB;float:left;" class="open-selector-endpoint">http://your-open-id/</span> ');
        } else {
            openid_block.after('<div>' + html_label + select_html + '<label id="open-selector-info" class="note"></label></div> <div id="open-selector-user-block" style="display:none;"><label id="open-selector-username-label" for="open-selector-username">Username</label>' + open_user_html + '<label class="open-selector-endpoint note" for="open-selector-username"></label></div>' + html_credits);
        }

        var open_selector = $('#open-selector');
        var open_user = $('#open-selector-username');
        var open_user_block = $('#open-selector-user-block');
        var info = $('#open-selector-info');
        var label = $('#open-selector-username-label');
        var span = $('.open-selector-endpoint');

        var provider, selected_provider, provider_id;
        var endpoint = '';
        var provider_options = [];

        // Fill the combo box with configured providers
        for (provider_id in providers) {
            provider = providers[provider_id];
            provider_options.push('<option id="' + provider_id + '" style="padding-left: 22px;background: url(' + provider.icon+ ') no-repeat 3px center">' + provider.name + '</option>');
        }
        open_selector.append(provider_options.join(''));
        // Select the first item (it has value 0)
        open_selector.val(0);

        var that = this;

        // Set what to do on each provider selecion
        open_selector.change(function(){
            endpoint = "";
            $('option', this).each(function(){
                if (this.selected) {
                    if (this.id) {
                        selected_provider = providers[this.id];
                        endpoint = selected_provider.url;

                        if (selected_provider.info) {
                            if (that.inline) {
                                open_user.val(selected_provider.info);
                            } else {
                                info.html(selected_provider.info);
                                info.show();
                            }
                        } else {
                            if (that.inline){
                                open_user.val('');
                            } else {
                                info.hide();
                            }
                        }

                        open_user.css('background-image', "url(" + selected_provider.icon + ")");
                        open_user.css('background-position', "3px center")
                        open_user.css('background-repeat', "no-repeat");
                        span.html(endpoint);
                        if ( selected_provider.ask_username) {
                            label.html(selected_provider.label ? selected_provider.label : "Username");

                            if (!that.inline) open_user_block.show();
                            open_user.removeAttr('disabled');
                            open_user.focus();
                        } else {
                            if (that.inline) {
                                open_user.attr('disabled', 'disabled');
                            } else {
                                open_user_block.hide();
                            }
                        }

                        if (this.id == 'openid') {
                            open_user.attr('name', 'openid');
                        }
                        else {
                            open_user.removeAttr('name');
                        }
                    } else {
                        // Not a provider selection
                        if (that.inline){
                            open_user.css('background', '#EEE');
                            open_user.attr('disabled', 'disabled');
                        } else {
                            open_user_block.hide();
                        }
                    }
                openid_box.val(endpoint.replace('{username}', open_user.val()));
                return; // Stop looping here
                }
            });
        });

        if (this.update_openid_url) {
            open_user.bind("blur focus keydown keyup click", function(){
                span.html(open_user.val()?
                    endpoint.replace('{username}', "<strong>" + (open_user.val()) + "</strong>"):endpoint);
                openid_box.val(endpoint.replace('{username}', open_user.val()));
            });
        }

        // Convert the endpoint to the OpenID identifier before submiting()
        var button_action = button.click;
        button.click(function(){
            endpoint = endpoint.replace('{username}', open_user.val());
            openid_box.val(endpoint);
            button_action();
        });

    }
};

